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

    let mut result = 0;
    let mut n = N;

    while n > 0 {
        n /= K;
        result += 1;
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
                r#"11 2
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1010101 10
"#
            ),
            r#"7"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"314159265 3
"#
            ),
            r#"18"#
        );
    }
}
