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
        A: isize,
        B: isize,
    }

    let result = if A - B * 2 > 0 { A - B * 2 } else { 0 };

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"12 4
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"20 15
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"20 30
"#
            ),
            r#"0"#
        );
    }
}
