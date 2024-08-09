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
        M: usize,
        D: usize,
        y: usize,
        m: usize,
        d: usize,
    }

    let mut y = y;
    let mut m = m;
    let mut d = d + 1;

    if d > D {
        m += 1;
        d = 1;
    }

    if m > M {
        y += 1;
        m = 1;
    }

    format!("{} {} {}", y, m, d)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"12 30
2023 12 30
"#
            ),
            r#"2024 1 1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"36 72
6789 23 45
"#
            ),
            r#"6789 23 46"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"12 30
2012 6 20
"#
            ),
            r#"2012 6 21"#
        );
    }
}
