use std::cmp::{Ord, Ordering};
use std::iter::*;
use std::mem::replace;

/// Invariants:
///
/// 1. Every red node has no red children.
/// 2. Every path from root to leaf contains exactly same number of black nodes.
pub struct RedBlackTree<TKey, T> {
    root: Option<Box<Node<TKey, T>>>,
}

struct Node<TKey, T> {
    size: usize,
    color: Color,
    key: TKey,
    value: T,
    l: Option<Box<Node<TKey, T>>>,
    r: Option<Box<Node<TKey, T>>>,
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

    pub fn map<F: Fn(&TKey, &T) -> T>(&self, f: &F) -> Self {
        Node {
            key: self.key.clone(),
            value: f(&self.key, &self.value),
            l: self.l.as_ref().map(|ref l| Box::new(l.map(f))),
            r: self.r.as_ref().map(|ref r| Box::new(r.map(f))),
            ..*self
        }
    }

    pub fn insert(mut self: Box<Self>, key: TKey, value: T) -> Box<Self> {
        match self.key.cmp(&key) {
            Ordering::Equal => {
                panic!("duplicated key");
            }
            Ordering::Less => self,
            Ordering::Greater => {
                let mut l = replace(&mut self.l, None);
                let mut l = match l {
                    None => Box::new(Node::new(Color::Red, key, value)),
                    Some(l) => l.insert(key, value).balance(),
                };
                self.l = Some(l);
                self.balance()
            }
        }
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
                //             self
                //            /    \
                //          [l]     r
                //         /  \
                //     [ll]    lr
                // to:
                //            l
                //           /  \
                //       [ll]   [self]
                //              /     \
                //            lr       r
                // where [_] is red.

                let mut l = replace(&mut self.l, None).unwrap();
                let mut lr = replace(&mut l.r, None);

                self.color = R;
                l.color = B;

                self.l = lr;
                l.r = Some(self);
                l
            }
            (B, (Some(R), _, Some(R)), _) => {
                // Rotate from:
                //          self
                //         /    \
                //        l      r
                //       /  \
                //     ll    lr
                //          /  \
                //        lrl   lrr
                // to:
                //           lr
                //         /    \
                //        l      self
                //      /   \    /   \
                //     ll   lrl lrr   r

                let mut l = replace(&mut self.l, None).unwrap();
                let mut lr = replace(&mut l.r, None).unwrap();
                let mut lrl = replace(&mut lr.l, None);
                let mut lrr = replace(&mut lr.r, None);

                self.color = R;
                lr.color = B;

                self.l = lrr;
                l.r = lrl;
                lr.l = Some(l);
                lr.r = Some(self);

                lr
            }
            (B, _, (Some(R), Some(R), _)) => self,
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
        self.root = Some(root.unwrap().insert(key, value));
    }

    fn balance(&mut self) {}

    pub fn map<F: Fn(&TKey, &T) -> T>(&self, f: F) -> Self {
        match self.root {
            None => Self::new(),
            Some(ref root) => RedBlackTree {
                root: Some(Box::new(root.map(&f))),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, RedBlackTree};

    #[test]
    fn test_insert() {
        let mut t = RedBlackTree::new();

        t.insert(3, "b");
        t.insert(2, "a");

        let root = t.root.as_ref().unwrap();
        assert_eq!(Color::Black, root.color);

        assert_eq!("b", root.value);

        let l = root.l.as_ref().unwrap();
        assert_eq!(Color::Red, l.color);
        assert_eq!("a", l.value);
    }
}
