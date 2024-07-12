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
        K: usize,
    }

    let mut result = vec![];

    for n in A..=B {
        if n < A + K || B - K < n {
            result.push(n);
        }
    }

    result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 8 2
"#
            ),
            r#"3
4
7
8"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 8 3
"#
            ),
            r#"4
5
6
7
8"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 9 100
"#
            ),
            r#"2
3
4
5
6
7
8
9"#
        );
    }
}
