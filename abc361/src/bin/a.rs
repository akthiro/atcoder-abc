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
        K: usize,
        X: usize,
        A: [usize; N],
    }

    let mut a = A;
    a.insert(K, X);

    a.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 3 7
2 3 5 11
"#
            ),
            r#"2 3 5 7 11"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 1 100
100
"#
            ),
            r#"100 100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 8 3
9 9 8 2 4 4 3 5
"#
            ),
            r#"9 9 8 2 4 4 3 5 3"#
        );
    }
}
