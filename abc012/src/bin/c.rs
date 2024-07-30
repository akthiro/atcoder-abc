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
    }

    let mut total = 0;

    for i in 1..=9 {
        for j in 1..=9 {
            total += i * j;
        }
    }

    let mut result = vec![];

    for i in 1..=9 {
        for j in 1..=9 {
            if N + (i * j) == total {
                result.push(format!("{} x {}", i, j));
            }
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2013
"#
            ),
            r#"2 x 6
3 x 4
4 x 3
6 x 2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2024
"#
            ),
            r#"1 x 1"#
        );
    }
}
