#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        _N: usize,
        M: usize,
        S: Chars,
    }

    let mut enable_plain = M;
    let mut enable_logo = 0;

    let mut result = 0;

    for c in S {
        if c == '0' {
            enable_plain = M;
            enable_logo = result;
        }

        if c == '1' {
            if enable_plain >= 1 {
                enable_plain -= 1;
                continue;
            }

            if enable_logo >= 1 {
                enable_logo -= 1;
                continue;
            }

            result += 1;
        }

        if c == '2' {
            if enable_logo >= 1 {
                enable_logo -= 1;
                continue;
            }

            result += 1;
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
                r#"6 1
112022
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 1
222
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 1
01
"#
            ),
            r#"0"#
        );
    }
}
