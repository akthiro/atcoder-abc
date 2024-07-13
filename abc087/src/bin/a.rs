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
        A: usize,
        B: usize,
    }

    let result = (X - A) - (B * ((X - A) / B));

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1234
150
100
"#
            ),
            r#"84"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1000
108
108
"#
            ),
            r#"28"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"579
123
456
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"7477
549
593
"#
            ),
            r#"405"#
        );
    }
}
