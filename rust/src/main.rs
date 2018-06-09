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

/// Vertex index.
type VI = usize;

/// Edge index.
type EI = usize;

#[derive(Clone, Debug)]
struct Vertex {
    id: VI,
    edges: Vec<(VI, EI)>,

    dfs_parent: Option<(VI, EI)>,
    dfs_children: Vec<(VI, EI)>,
    dfs_ord: usize,
    dfs_done: bool,
    /// これが属する2連結成分の代表元
    comp_repr: Option<VI>,
    /// これが属する2連結成分の位数
    comp_size: usize,
}

/// bidirected, u < v
#[derive(Clone, Debug)]
struct Edge {
    id: EI,
    u: VI,
    v: VI,
    dfs_done: bool,
    imos: i32,
}

#[derive(Debug)]
struct Graph {
    vs: Vec<Vertex>,
    es: Vec<Edge>,
    next_ord: usize,
}

impl Graph {
    fn is_bridge(&self, e: EI) -> bool {
        self.es[e].imos == 0
    }

    fn dfs_walk(&mut self, v: usize, parent: Option<(VI, EI)>) {
        if self.vs[v].dfs_done {
            return;
        }

        self.vs[v].dfs_done = true;
        self.vs[v].dfs_ord = self.next_ord;
        self.next_ord += 1;

        if let Some((p, e)) = parent {
            self.es[e].dfs_done = true;
            self.vs[v].dfs_parent = Some((p, e));
            self.vs[p].dfs_children.push((v, e));
        }

        // NOTE: Can't iterate over edges because it may change in dfs_walk. Actually not, though.
        let N = self.vs[v].edges.len();
        for i in 0..N {
            let (w, e) = self.vs[v].edges[i];
            self.dfs_walk(w, Some((v, e)));
        }
    }

    fn dfs(&mut self) {
        for v in 0..self.vs.len() {
            self.dfs_walk(v, None);
        }
    }

    fn imos_first(&mut self) {
        for ei in 0..self.es.len() {
            let (a, d) = {
                let edge = &self.es[ei];
                if edge.dfs_done {
                    continue;
                }

                if self.vs[edge.u].dfs_ord < self.vs[edge.v].dfs_ord {
                    (edge.u, edge.v)
                } else {
                    (edge.v, edge.u)
                }
            };

            if let Some((_, e)) = self.vs[a].dfs_parent {
                debug!(e, -1);
                self.es[e].imos -= 1;
            }
            if let Some((_, e)) = self.vs[d].dfs_parent {
                debug!(e, 1);
                self.es[e].imos += 1;
            }
        }
    }

    // dfsツリーにおいてvから伸びている各エッジがブリッジかどうか判定して、
    // imos値の総和を返す
    fn imos_acc_walk(&mut self, v: usize) -> i32 {
        // imos (v -> **)
        let mut sum = 0;
        for ci in 0..self.vs[v].dfs_children.len() {
            let (w, e) = self.vs[v].dfs_children[ci];

            // imos(v -> w -> **) = imos(v -> w) + imos(w -> **)
            self.es[e].imos += self.imos_acc_walk(w);

            sum += self.es[e].imos;
        }
        sum
    }

    fn measure_component_walk(&mut self, v: usize, repr: VI) -> usize {
        self.vs[v].comp_repr = Some(repr);
        let mut size = 1;
        for ei in 0..self.vs[v].edges.len() {
            let (w, e) = self.vs[v].edges[ei];

            if self.is_bridge(e) || self.vs[w].comp_repr.is_some() {
                continue;
            }
            size += self.measure_component_walk(w, repr);
        }
        size
    }

    fn measure_components(&mut self) {
        for v in 0..self.vs.len() {
            self.vs[v].comp_size = match self.vs[v].comp_repr {
                None => self.measure_component_walk(v, v),
                Some(repr) => self.vs[repr].comp_size,
            }
        }
    }

    fn solve(&mut self) -> Vec<usize> {
        // 1. ブリッジを検出する。
        // 深さ優先ツリーを作る。
        // dfsツリーの中で、後退辺 (dfsツリーに含まれていない辺) は (両端の一方が他方の祖先なので、祖先を a、子孫を d とおく。d から親への辺を +1、a から親への辺を -1 する。
        // dfsツリーのリーフからルートに向かってエッジにつけた数値の累積和をとっていく。累積和が 0 のエッジがブリッジ。

        // 2. 2連結成分分解
        // ブリッジでないエッジだけで連結成分分解すればいい

        // 3. 各頂点につき、それが属す成分の位数が求める都市の個数。

        self.dfs();
        self.imos_first();
        self.imos_acc_walk(0);
        self.measure_components();
        debug!(self);

        self.vs.iter().map(|v| v.comp_size).collect::<Vec<_>>()
    }
}

pub fn main() {
    let w = rw::<usize>();
    let (N, K, L) = (w[0], w[1], w[2]);
    let M = K + L;

    let mut vs = (0..N)
        .map(|v| Vertex {
            id: v,
            edges: Vec::new(),

            dfs_parent: None,
            dfs_children: Vec::new(),
            dfs_ord: 0,
            dfs_done: false,
            comp_repr: None,
            comp_size: 0,
        })
        .collect::<Vec<_>>();

    let mut es = (0..M)
        .map(|e| Edge {
            id: e,
            u: 0,
            v: 0,
            dfs_done: false,
            imos: 0,
        })
        .collect::<Vec<_>>();

    for e in 0..M {
        let w = rw::<usize>();
        let (u, v) = (w[0] - 1, w[1] - 1);
        es[e].u = u;
        es[e].v = v;
        vs[u].edges.push((v, e));
        vs[v].edges.push((u, e));
    }

    let sizes = Graph {
        vs: vs,
        es: es,
        next_ord: 0,
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
