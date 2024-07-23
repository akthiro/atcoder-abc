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
        A: usize,
        B: usize,
    }

    let result = (B as f64 / 100.0) * A as f64;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"45 200
"#
            ),
            r#"90"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"37 450
"#
            ),
            r#"166.5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"0 1000
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"50 0
"#
            ),
            r#"0"#
        );
    }
}
