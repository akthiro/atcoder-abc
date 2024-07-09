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
        Y: usize,
        Z: usize,
    }

    let (a, b) = (Y, X);
    let (a, c) = (Z, a);

    format!("{} {} {}", a, b, c)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1 2 3
"#
            ),
            r#"3 1 2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"100 100 100
"#
            ),
            r#"100 100 100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"41 59 31
"#
            ),
            r#"31 41 59"#
        );
    }
}
