#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::io::*;
use std::ops::*;
use std::*;

// -----------------------------------------------
// Framework
// -----------------------------------------------

#[allow(unused)]
fn rl() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_owned()
}

#[allow(unused)]
fn rw<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|word| T::from_str(word).unwrap())
        .collect()
}

trait IteratorExt: Iterator + Sized {
    fn vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<T: Iterator> IteratorExt for T {}

#[allow(unused)]
macro_rules! debug {
    ($($arg:expr),*) => {
        #[cfg(debug_assertions)]
        {
            let entries = &[
                $((
                    &stringify!($arg).to_string() as &fmt::Debug,
                    &($arg) as &fmt::Debug,
                )),*
            ];
            eprintln!("{:#?}", DebugMap(entries));
        }
    };
}

#[allow(unused)]
struct DebugMap<'a>(&'a [(&'a fmt::Debug, &'a fmt::Debug)]);

impl<'a> std::fmt::Debug for DebugMap<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut m = fmt.debug_map();
        for &(key, value) in self.0.iter() {
            m.entry(key, value);
        }
        m.finish()
    }
}

// -----------------------------------------------
// Polyfill
// -----------------------------------------------

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[allow(unused)]
macro_rules! eprintln {
    ($($arg:expr),*) => { _eprintln(format_args!($($arg),*)) }
}

fn _eprintln(args: fmt::Arguments) {
    let err = std::io::stderr();
    let mut err = err.lock();
    err.write_fmt(args).unwrap();
    err.write(b"\n").unwrap();
}

// -----------------------------------------------
// Solution
// -----------------------------------------------

type Vertex = usize;
type EdgeId = usize;

#[derive(Clone, Debug)]
struct Edge {
    id: EdgeId,
    u: usize,
    v: usize,
}

#[derive(Debug)]
struct Graph {
    ad: Vec<Vec<Edge>>,
    edges: Vec<Edge>,
}

#[derive(Clone, Debug)]
struct DfsTreeVertex {
    parent_edge: Option<EdgeId>,
    children: Vec<Vertex>,
    ord: usize,
    done: bool,
}

#[derive(Debug)]
struct DfsTree<'a> {
    graph: &'a Graph,
    vertices: Vec<DfsTreeVertex>,
    edge_is_used: Vec<bool>,
    nextOrd: usize,
}

impl<'a> DfsTree<'a> {
    pub fn new(graph: &Graph) -> DfsTree {
        let mut t = DfsTree {
            graph: graph,
            vertices: vec![
                DfsTreeVertex {
                    parent_edge: None,
                    children: Vec::new(),
                    ord: 0,
                    done: false,
                };
                graph.ad.len()
            ],
            edge_is_used: vec![false; graph.edges.len()],
            nextOrd: 0,
        };
        t.walk(0, None);
        t
    }

    fn walk(&mut self, v: usize, parent_edge: Option<&Edge>) {
        if self.vertices[v].done {
            return;
        }

        self.vertices[v].done = true;
        self.vertices[v].ord = self.nextOrd;
        self.nextOrd += 1;

        if let Some(edge) = parent_edge {
            self.edge_is_used[edge.id] = true;
            self.vertices[v].parent_edge = Some(edge.id);
            self.vertices[v].children.push(edge.v);
        }

        for edge in self.graph.ad[v].iter() {
            self.walk(edge.v, Some(edge));
        }
    }
}

impl Graph {
    fn dfs(&self) -> DfsTree {
        DfsTree::new(self)
    }

    fn solve(&self) -> Vec<usize> {
        // 1. ブリッジを検出する。
        // 深さ優先ツリーを作る。
        // dfsツリーの中で、後退辺 (dfsツリーに含まれていない辺) は (両端の一方が他方の祖先なので、祖先を a、子孫を d とおく。d から親への辺を +1、a から親への辺を -1 する。
        // dfsツリーのリーフからルートに向かってエッジにつけた数値の累積和をとっていく。累積和が 0 のエッジがブリッジ。

        // 2. 2連結成分分解
        // ブリッジでないエッジだけで連結成分分解すればいい

        // 3. 各頂点につき、それが属す成分の位数が求める都市の個数。

        let dfs_tree = self.dfs();
        let mut dp = vec![0; self.edges.len()];
        for e in self.edges.iter() {
            if dfs_tree.edge_is_used[e.id] {
                continue;
            }

            let (a, d) = if dfs_tree.vertices[e.u].ord < dfs_tree.vertices[e.v].ord {
                (e.u, e.v)
            } else {
                (e.v, e.u)
            };

            if let Some(e) = dfs_tree.vertices[a].parent_edge {
                debug!(e, -1);
                dp[e] -= 1;
            }
            if let Some(e) = dfs_tree.vertices[d].parent_edge {
                debug!(e, 1);
                dp[e] += 1;
            }
        }
        debug!(dfs_tree, dp);

        fn go(v: usize, tree: &DfsTree, dp: &mut Vec<i32>) -> i32 {
            let mut sum = 0;
            for &child in tree.vertices[v].children.iter() {
                if let Some(e) = tree.vertices[v].parent_edge {
                    sum += dp[e];
                }
                sum += go(child, tree, dp);
            }
            sum
        }
        go(0, &dfs_tree, &mut dp);
        debug!(dp);

        // dp[e] == 0 <=> e: bridge
        let is_bridge = move |e: EdgeId| dp[e] == 0;

        fn component_size<F: Fn(EdgeId) -> bool>(
            v: usize,
            root: usize,
            graph: &Graph,
            roots: &mut Vec<Option<usize>>,
            is_bridge: &F,
        ) -> usize {
            roots[v] = Some(root);
            let mut size = 1;

            for e in graph.ad[v].iter() {
                if is_bridge(e.id) || roots[e.v].is_some() {
                    continue;
                }

                size += component_size(e.v, root, graph, roots, is_bridge);
            }

            size
        }
        let mut sizes = vec![0; self.ad.len()];
        let mut roots = vec![None; self.ad.len()];
        for v in 0..self.ad.len() {
            if roots[v].is_some() {
                continue;
            }

            sizes[v] = component_size(v, v, self, &mut roots, &is_bridge);
        }
        for v in 0..self.ad.len() {
            sizes[v] = sizes[roots[v].unwrap()];
        }

        sizes
    }
}

pub fn main() {
    let w = rw::<usize>();
    let (N, K, L) = (w[0], w[1], w[2]);

    let mut G = vec![Vec::new(); N];
    let mut edges = Vec::new();

    for _ in 0..(K + L) {
        let w = rw::<usize>();
        let (u, v) = (w[0] - 1, w[1] - 1);

        let e = Edge {
            id: edges.len(),
            u: u,
            v: v,
        };
        edges.push(e.clone());
        G[u].push(e);

        let e = Edge {
            id: edges.len(),
            u: v,
            v: u,
        };
        G[v].push(e.clone());
        edges.push(e);
    }

    let sizes = Graph {
        ad: G,
        edges: edges,
    }.solve();

    println!(
        "{}",
        sizes
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(7, 1 + 2 * 3);
    }
}
