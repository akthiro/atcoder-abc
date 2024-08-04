#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{input, marker::Chars, source::once::OnceSource};
use std::{
    cmp,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: usize,
        M: usize,
        S: [Chars; N],
    }

    let mut result = N;

    for bits in (0..N).map(|_| [true, false]).multi_cartesian_product() {
        let mut oks = vec![false; M];

        for (i, bit) in bits.iter().enumerate() {
            if !bit {
                continue;
            }

            for (j, c) in S[i].iter().enumerate() {
                if *c == 'x' {
                    continue;
                }

                oks[j] = true;
            }
        }

        if oks.iter().all(|ok| *ok) {
            result = cmp::min(result, bits.iter().filter(|bit| **bit).count());
        }
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 5
oooxx
xooox
xxooo
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 2
oo
ox
xo
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 6
xxoxxo
xxoxxx
xoxxxx
xxxoxx
xxoooo
xxxxox
xoxxox
oxoxxo
"#
            ),
            r#"3"#
        );
    }
}
