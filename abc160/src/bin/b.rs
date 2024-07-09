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
        X: usize,
    }

    let mut result = 0;
    let mut x = X;

    result += (x / 500) * 1000;
    x %= 500;

    result += (x / 5) * 5;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1024
"#
            ),
            r#"2020"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"0
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1000000000
"#
            ),
            r#"2000000000"#
        );
    }
}
