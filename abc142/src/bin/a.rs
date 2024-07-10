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
    }

    let odds = (N + 1) / 2;
    let result = odds as f64 / N as f64;

    format!("{:.10}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
"#
            ),
            r#"0.5000000000"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
"#
            ),
            r#"0.6000000000"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1
"#
            ),
            r#"1.0000000000"#
        );
    }
}
