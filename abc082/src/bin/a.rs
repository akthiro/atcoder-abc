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
        a: usize,
        b: usize,
    }

    let result = ((a as f64 + b as f64) / 2.0).ceil();

    format!("{:.0}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1 3
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7 4
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5 5
"#
            ),
            r#"5"#
        );
    }
}
