#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::io::{self, Read};

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
    }

    let mut result = N;

    for _ in 0..K {
        if result % 200 == 0 {
            result /= 200;
        } else {
            result = result * 1000 + 200;
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
                r#"2021 4
"#
            ),
            r#"50531"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"40000 2
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8691 20
"#
            ),
            r#"84875488281"#
        );
    }
}
