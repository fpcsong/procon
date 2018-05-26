#![allow(unused_imports)]
#![allow(non_snake_case)]

use procon::*;
use std::cmp::{max, min};
use std::collections::*;
use std::ops::*;
use std::*;

fn solve(K: usize, Q: usize, A: Vec<i64>) -> i64 {
    let mut score = 1_000_000_000;

    for y in A.iter().cloned() {
        // 最良のスコア X - Y が Y >= y で与えられるケースだけ考える。
        // y 未満の要素を含む部分列は選択できない。
        // y 未満の要素で数列を切断して、各部分を島と呼ぶ。
        // 各島について操作を行うとき、長さ K 以上なら、必ず最小値を消すことができる。
        // 各操作において、長さ K 以上の島に含まれる要素の中で最小のもの、を消すのが常に最善。
        // X が最小化されてベスト。

        let mut islands = Vec::new();
        let mut island = Vec::new();
        for a in A.iter().cloned() {
            if a < y {
                if island.len() >= K {
                    islands.push(island);
                    island = Vec::new();
                }
                continue;
            }

            island.push(a);
        }

        if island.len() >= K {
            islands.push(island);
        }

        //println!("y={} {:?}", y, islands);

        // Q 番目に小さい数を探す
        let mut cand = Vec::new();

        // 長さ K 未満の島からはドロップできないので、小さいものから (N-K+1) 個しか使えない
        for mut island in islands {
            island.sort();
            for i in 0..(island.len() + 1 - K) {
                cand.push(island[i]);
            }
        }

        if cand.len() < Q {
            // Y >= y になる操作が存在しない
            continue;
        }

        cand.sort();

        let y = cand[0];
        let x = cand[Q - 1];

        score = min(score, x - y);
    }
    score
}

pub fn main() {
    let words = read_words::<usize>();
    let (_, K, Q) = (words[0], words[1], words[2]);
    let A = read_words::<i64>();

    println!("{}", solve(K, Q, A));
    return;
}

pub mod procon {
    use std;
    use std::collections::*;
    use std::io;
    use std::mem;
    use std::str::FromStr;

    pub fn read_line() -> String {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim_right().to_owned()
    }

    pub fn read_words<T>() -> Vec<T>
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

    pub fn read_vec(len: usize) -> Vec<String> {
        let mut vec = Vec::new();
        while vec.len() < len {
            let line = read_line();
            for word in line.split_whitespace() {
                vec.push(word.to_owned());
            }
        }
        assert!(vec.len() == len);
        vec
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_case1() {
        assert_eq!(1, solve(3, 2, vec![4, 3, 1, 5, 2]));
    }

    #[test]
    pub fn test_case2() {
        assert_eq!(7, solve(1, 6, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]));
    }

    #[test]
    pub fn test_case3() {
        assert_eq!(
            451211184,
            solve(
                7,
                5,
                vec![
                    24979445, 861648772, 623690081, 433933447, 476190629, 262703497, 211047202,
                    971407775, 628894325, 731963982, 822804784,
                ]
            )
        );
    }

}
