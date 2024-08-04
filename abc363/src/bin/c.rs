#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::io::{self, Read};
use superslice::Ext;

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: usize,
        K: usize,
        S: Chars,
    }

    let mut cs = S;
    cs.sort();

    let mut result = 0;

    loop {
        let mut not_contains = true;

        for i in 0..=N - K {
            let mut contains = true;

            for j in 1..K {
                if cs[i + j] != cs[i + K - 1 - j] {
                    contains = false;
                    break;
                }
            }

            if contains {
                not_contains = false;
                break;
            }
        }

        if not_contains {
            result += 1;
        }

        if !cs.next_permutation() {
            break;
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
                r#"3 2
aab
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 3
zzyyx
"#
            ),
            r#"16"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 5
abcwxyzyxw
"#
            ),
            r#"440640"#
        );
    }
}
