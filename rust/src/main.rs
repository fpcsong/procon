use std::cmp::max;
use std::io::BufRead;

fn diff(s: &[char], t: &[char]) -> Vec<(char, bool)> {
    let sn = s.len();
    let tn = t.len();

    // dp[i][j] = k <=> s[..i], t[..j] の最長部分共通列の末尾が s[k - 1]
    let mut dp = vec![vec![0; tn + 1]; sn + 1];

    for si in 0..sn + 1 {
        for ti in 0..tn + 1 {
            if si < sn && ti < tn && s[si] == t[ti] {
                dp[si + 1][ti + 1] = si + 1;
            }

            if si + 1 < sn + 1 {
                dp[si + 1][ti] = max(dp[si + 1][ti], dp[si][ti]);
            }

            if ti + 1 < tn + 1 {
                dp[si][ti + 1] = max(dp[si][ti + 1], dp[si][ti]);
            }
        }
    }

    let mut buf = Vec::new();
    {
        let mut si = sn;
        let mut ti = tn;
        while !(si == 0 && ti == 0) {
            let k = dp[si][ti];

            if si > 0 && ti > 0 && s[si - 1] == t[ti - 1] {
                buf.push((s[si - 1], true));
                si -= 1;
                ti -= 1;
            } else if si > 0 && k == dp[si - 1][ti] {
                buf.push((s[si - 1], false));
                si -= 1;
            } else if ti > 0 && k == dp[si][ti - 1] {
                buf.push((t[ti - 1], false));
                ti -= 1;
            } else {
                unreachable!()
            }
        }
    }

    buf.into_iter().rev().collect::<Vec<_>>()
}

fn pattern_from_edit(edit: &[(char, bool)]) -> Vec<char> {
    let mut pattern = Vec::new();
    {
        let mut prev = true;
        for &(c, common) in edit {
            if common && !prev {
                pattern.push('*');
            }

            if common {
                pattern.push(c);
            }

            prev = common;
        }

        if !prev {
            pattern.push('*');
        }
    }
    pattern
}

fn normalize(pattern: &[char]) -> String {
    let mut buf = Vec::new();
    let mut prev = None;
    for &c in pattern {
        if c == '*' && Some(c) == prev {
            continue;
        }

        buf.push(c);
        prev = Some(c);
    }
    buf.into_iter().collect::<String>()
}

pub fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let ss = lock.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut pattern = ss[0].clone();
    for r in ss.into_iter().skip(1) {
        let edit = diff(&pattern, &r);
        pattern = pattern_from_edit(&edit);
    }

    println!("{}", normalize(&pattern));
}
