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

    let result = ((B as f64 / A as f64) * 1000.0).round() / 1000.0;

    format!("{:.3}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7 4
"#
            ),
            r#"0.571"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7 3
"#
            ),
            r#"0.429"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 1
"#
            ),
            r#"0.500"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"10 10
"#
            ),
            r#"1.000"#
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            solve(
                r#"1 0
"#
            ),
            r#"0.000"#
        );
    }
}
