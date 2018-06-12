#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::cell::*;
use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::io::*;
use std::marker::PhantomData;
use std::mem::*;
use std::ops::*;
use std::rc::*;
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
            eprintln!("{:?}", DebugMap(entries));
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

trait ClosureMut<X, Y> {
    fn call(&self, input: X) -> Y;
}

struct RecursiveClosureMut<X, Y, F>(UnsafeCell<F>, PhantomData<X>, PhantomData<Y>);

impl<X, Y, F> ClosureMut<X, Y> for RecursiveClosureMut<X, Y, F>
where
    F: FnMut(&ClosureMut<X, Y>, X) -> Y,
{
    fn call(&self, input: X) -> Y {
        let f = unsafe { &mut *self.0.get() };
        f(self, input)
    }
}

fn recurse<X, Y, F>(x: X, f: F) -> Y
where
    F: FnMut(&ClosureMut<X, Y>, X) -> Y,
{
    RecursiveClosureMut(UnsafeCell::new(f), PhantomData, PhantomData).call(x)
}

fn fixpoint<'a, X: 'a, Y: 'a, F>(f: F) -> Box<FnMut(X) -> Y + 'a>
where
    F: FnMut(&ClosureMut<X, Y>, X) -> Y + 'a,
{
    let f = RecursiveClosureMut(UnsafeCell::new(f), PhantomData, PhantomData);
    Box::new(move |x: X| f.call(x))
}

pub fn main() {
    let N = 7;
    let mut A = vec![vec![]; N];

    for &(u, v) in &[(1, 3), (3, 2), (3, 4), (5, 6)] {
        A[u].push(v);
        A[v].push(u);
    }

    let mut root = vec![None; N];

    for v in 0..N {
        recurse((v, v), |dfs, (v, r)| {
            if root[v].is_some() {
                return;
            }

            // It can borrow variables out of the closure.
            root[v] = Some(r);

            for &w in A[v].iter() {
                // Recursive call!
                dfs.call((w, r));
            }
        });
        /*
        let mut dfs = Y(
            |dfs: &mut FnMut((usize, usize)) -> (), (v, r): (usize, usize)| {
                if root[v].is_some() {
                    return;
                }

                // It can borrow variables out of the closure.
                root[v] = Some(r);

                for &w in A[v].iter() {
                    // Recursive call!
                    dfs((w, r));
                }
            },
        );
        dfs.apply((v, v));
        */
    }

    debug!(root);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(7, 1 + 2 * 3);
    }

    #[test]
    fn fact() {
        let f7 = recurse(7, |fact, n| if n == 0 { 1 } else { fact.call(n - 1) * n });
        assert_eq!(f7, 1 * 2 * 3 * 4 * 5 * 6 * 7);
    }

    #[test]
    fn memoization() {
        let mut memo = HashMap::new();
        let mut fib = {
            fixpoint(|fib, n: i32| {
                let e = memo.entry(n).or_insert_with(|| {
                    if n <= 1 {
                        1
                    } else {
                        fib.call(n - 1) + fib.call(n - 2)
                    }
                });
                *e
            })
        };
        assert_eq!(fib(0), 1);
        assert_eq!(fib(4), 5);
        assert_eq!(fib(5), 8);
        assert_eq!(fib(10), 89);
        assert_eq!(fib(20), 10946);
    }
}
