#![allow(unused)]
use std::cmp::{Ord, Ordering};
use std::mem::replace;

/// Invariants:
///
/// 1. Every red node has no red children.
/// 2. Every path from root to leaf contains exactly same number of black nodes.
pub struct RedBlackTree<TKey, T> {
    /// 0 if empty.
    root: usize,
    /// [0] is always invalid.
    node: Vec<Node<TKey, T>>,
}

struct Node<TKey, T> {
    /// The number of nodes in the subtree.
    /// 0 if invalid.
    size: usize,
    /// 0 if invalid.
    l: usize,
    /// 0 if invalid.
    r: usize,
    /// Black if invalid.
    color: Color,
    content: Option<(TKey, T)>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Color {
    Red,
    Black,
}

const R: Color = Color::Red;
const B: Color = Color::Black;

impl<TKey, T> Node<TKey, T>
where
    TKey: Ord + Clone,
{
    fn new(color: Color, key: TKey, value: T) -> Self {
        Node {
            size: 1,
            color: color,
            key: key,
            value: value,
            l: None,
            r: None,
        }
    }

    pub fn make_black(mut self: Box<Self>) -> Box<Self> {
        self.color = B;
        self
    }

    pub fn insert(mut self: Box<Self>, key: TKey, value: T) -> Box<Self> {
        match self.key.cmp(&key) {
            Ordering::Equal => {
                panic!("duplicated key");
            }
            Ordering::Less => {
                let mut r = replace(&mut self.r, None);
                let mut r = match r {
                    None => Box::new(Node::new(Color::Red, key, value)),
                    Some(r) => r.insert(key, value).balance(),
                };
                self.r = Some(r);
                self.update_size();
                self.balance()
            }
            Ordering::Greater => {
                let mut l = replace(&mut self.l, None);
                let mut l = match l {
                    None => Box::new(Node::new(Color::Red, key, value)),
                    Some(l) => l.insert(key, value).balance(),
                };
                self.l = Some(l);
                self.update_size();
                self.balance()
            }
        }
    }

    fn update_size(&mut self) {
        let ln = self.l.as_ref().map(|n| n.size).unwrap_or(0);
        let rn = self.r.as_ref().map(|n| n.size).unwrap_or(0);
        self.size = ln + rn + 1;
    }

    fn balance(mut self: Box<Self>) -> Box<Self> {
        fn colors<TKey, T>(
            node: &Option<Box<Node<TKey, T>>>,
        ) -> (Option<Color>, Option<Color>, Option<Color>) {
            match *node {
                None => (None, None, None),
                Some(ref node) => (
                    Some(node.color),
                    node.l.as_ref().map(|n| n.color),
                    node.r.as_ref().map(|n| n.color),
                ),
            }
        }

        match (self.color, colors(&self.l), colors(&self.r)) {
            (B, (Some(R), Some(R), _), _) => {
                // Rotate from:
                //             (self)
                //             /    \
                //           [l]
                //           /  \
                //       [ll]    lr
                // to:
                //           [l]
                //           /  \
                //       (ll)  (self)
                //             /    \
                //            lr
                // where [_] is red and (_) is black.

                let mut l = replace(&mut self.l, None).unwrap();
                let mut lr = replace(&mut l.r, None);

                l.l.as_mut().unwrap().color = B;

                self.l = lr;
                self.update_size();

                l.r = Some(self);
                l.update_size();

                l
            }
            (B, (Some(R), _, Some(R)), _) => {
                // Rotate from:
                //         (self)
                //         /    \
                //       [l]
                //       /  \
                //          [lr]
                //          /  \
                //        lrl   lrr
                // to:
                //          [lr]
                //         /    \
                //       (l)    (self)
                //      /   \    /   \
                //          lrl lrr

                let mut l = replace(&mut self.l, None).unwrap();
                let mut lr = replace(&mut l.r, None).unwrap();
                let mut lrl = replace(&mut lr.l, None);
                let mut lrr = replace(&mut lr.r, None);

                l.color = B;

                self.l = lrr;
                self.update_size();

                l.r = lrl;
                l.update_size();

                lr.l = Some(l);
                lr.r = Some(self);
                lr.update_size();

                lr
            }
            (B, _, (Some(R), _, Some(R))) => {
                // Rotate from:
                //      (self)
                //      /    \
                //     l     [r]
                //           /  \
                //         rl   [rr]
                // to:
                //           [r]
                //          /   \
                //      (self)  (rr)
                //      /    \
                //     l      rl

                let mut r = replace(&mut self.r, None).unwrap();
                let mut rl = replace(&mut r.l, None);

                r.r.as_mut().unwrap().color = B;

                self.r = rl;
                self.update_size();

                r.l = Some(self);
                r.update_size();

                r
            }
            (B, _, (Some(R), Some(R), _)) => {
                // Rotate from:
                //         (self)
                //         /    \
                //              [r]
                //              /  \
                //            [rl]
                //            /  \
                //          rll   rlr
                // to:
                //            [rl]
                //           /    \
                //      (self)      (r)
                //       /   \     /   \
                //          rll   rlr

                let mut r = replace(&mut self.r, None).unwrap();
                let mut rl = replace(&mut r.l, None).unwrap();
                let mut rll = replace(&mut rl.l, None);
                let mut rlr = replace(&mut rl.r, None);

                r.color = B;

                self.r = rll;
                self.update_size();

                r.l = rlr;
                r.update_size();

                rl.l = Some(self);
                rl.r = Some(r);
                rl.update_size();

                rl
            }
            _ => self,
        }
    }
}

impl<TKey, T> RedBlackTree<TKey, T>
where
    TKey: Ord + Clone,
    T: Clone,
{
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn size(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref root) => root.size,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, key: TKey, value: T) {
        if self.is_empty() {
            self.root = Some(Box::new(Node::new(Color::Black, key, value)));
            return;
        }

        let root = replace(&mut self.root, None);
        self.root = Some(root.unwrap().insert(key, value).make_black())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::{Color, Node, RedBlackTree};
    use std;
    use std::cmp::max;
    use std::io::*;
    use std::iter::repeat;

    fn draw<K: ToString, V: ToString>(tree: &RedBlackTree<K, V>) -> String {
        match tree.root {
            None => "(empty)".to_owned(),
            Some(ref node) => draw_node(node).1.join("\n"),
        }
    }

    fn draw_node<K: ToString, V: ToString>(node: &Node<K, V>) -> (usize, Vec<String>) {
        let k = node.key.to_string();
        let v = node.value.to_string();
        let s = match (node.color, node.size) {
            (Color::Black, 1) => format!("({})", k),
            (Color::Red, 1) => format!("[{}]", k),
            (Color::Black, size) => format!("({} #{})", k, size),
            (Color::Red, size) => format!("[{} #{}])", k, size),
        };
        if node.l.is_none() && node.r.is_none() {
            return (s.len(), vec![s]);
        }

        let (lw, l) = node.l.as_ref().map(|n| draw_node(n)).unwrap_or((0, vec![]));
        let (rw, r) = node.r.as_ref().map(|n| draw_node(n)).unwrap_or((0, vec![]));

        let replicate = |n: usize, x: char| repeat(x).take(n).collect::<String>();
        let pad = |s: &str, width: usize| {
            s.chars()
                .chain(repeat(' ').take(width - s.len()))
                .collect::<String>()
        };
        let shift =
            |s: &str, width: usize| repeat(' ').take(width).chain(s.chars()).collect::<String>();

        let w = max(lw + rw, s.len()) + 1;

        let root_row = shift(&s, (w - s.len()) / 2);
        let edge_row = format!(
            "{}{}{}{}",
            replicate(lw / 2, ' '),
            if !l.is_empty() { '/' } else { ' ' },
            replicate((lw + 1) / 2 + rw / 2, ' '),
            if !r.is_empty() { '\\' } else { ' ' }
        );
        let h = max(l.len(), r.len());
        let v = l.into_iter()
            .chain(repeat(String::default()))
            .zip(r.into_iter().chain(repeat(String::default())))
            .take(h)
            .map(|(ls, rs)| format!("{}{}", ls, shift(&rs, w - rw - ls.len())))
            .collect::<Vec<_>>();

        let lines = vec![root_row, edge_row]
            .into_iter()
            .chain(v.into_iter())
            .map(|line| line.trim_right().to_owned())
            .collect::<Vec<_>>();

        (w, lines)
    }

    #[test]
    fn test_draw() {
        let mut t = RedBlackTree::new();

        for n in 1..17 {
            t.insert(n, (('a' as u8 + n) as char).to_string());
        }

        assert_eq!(
            r#"               (8 #16)
       /                   \
    (4 #7)             (12 #8)
   /       \        /          \
(2 #3)  (6 #3)  (10 #3)     (14 #4)
 /   \   /   \   /    \    /      \
(1) (3) (5) (7) (9) (11) (13) (15 #2)
                                 \
                                  [16]"#,
            draw(&t)
        );
    }
}
