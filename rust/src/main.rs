#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::cmp::{max, min, Ordering};
use std::collections::*;
use std::fmt::{Debug, Formatter, Write as FmtWrite};
use std::io::{stderr, stdin, BufRead, Write};
use std::mem::{replace, swap};
use std::ops::*;
use std::rc::Rc;

// -----------------------------------------------
// Framework
// -----------------------------------------------

#[allow(unused_macros)]
macro_rules! read {
    ([$t:ty] ; $n:expr) =>
        ((0..$n).map(|_| read!([$t])).collect::<Vec<_>>());
    ($($t:ty),+ ; $n:expr) =>
        ((0..$n).map(|_| read!($($t),+)).collect::<Vec<_>>());
    ([$t:ty]) =>
        (rl().split_whitespace().map(|w| w.parse().unwrap()).collect::<Vec<$t>>());
    ($t:ty) =>
        (rl().parse::<$t>().unwrap());
    ($($t:ty),*) => {{
        let buf = rl();
        let mut w = buf.split_whitespace();
        ($(w.next().unwrap().parse::<$t>().unwrap()),*)
    }};
}

macro_rules! object {
    (
        $obj_name:ident (
            $($arg_name:ident : $arg_ty:ty),* $(,)*
        ) {
            $(let $var_pat:pat = $var_init:expr;)*
            $(val $field_name:ident : $field_ty:ty = $field_init:expr;)*
        }
    ) => {
        struct $obj_name {
            $($field_name : $field_ty),*
        }

        impl $obj_name {
            fn new($($arg_name : $arg_ty),*) -> $obj_name {
                $(let $var_pat = $var_init;)*
                $(let $field_name : $field_ty = $field_init;)*
                $obj_name {
                    $($field_name : $field_name),*
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($arg:expr),*) => {
        #[cfg(debug_assertions)]
        $(writeln!(stderr(), "{} = {:?}", stringify!($arg), $arg).unwrap());*
    };
}

#[allow(dead_code)]
fn rl() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_owned()
}

trait IteratorExt: Iterator + Sized {
    fn vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}

impl<T: Iterator> IteratorExt for T {}

// -----------------------------------------------
// Solution
// -----------------------------------------------

// object!(Solver() {
//     let a = 0;
//     val N: usize = read!(usize);
// });

// impl Solver {
//     fn run(mut self) {
//         println!("{}", 0);
//     }
// }

struct Solver {
    N: usize,
}

impl Solver {
    fn new() -> Solver {
        let N = read!(usize);
        Solver { N: N }
    }

    fn run(mut self) {
        println!("{}", 0)
    }
}

fn main() {
    Solver::new().run()
}
