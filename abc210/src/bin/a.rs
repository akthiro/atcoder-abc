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
        A: usize,
        X: usize,
        Y: usize,
    }

    if N <= A {
        format!("{}", N * X)
    } else {
        format!("{}", A * X + (N - A) * Y)
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 3 20 15
"#
            ),
            r#"90"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10 10 100 1
"#
            ),
            r#"1000"#
        );
    }
}
