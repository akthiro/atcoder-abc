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
        d: [usize; N],
    }

    let mut d = d;
    d.sort();

    let result = d[N / 2] - d[N / 2 - 1];

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"6
9 1 4 4 6 7
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8
9 1 14 5 5 4 4 14
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"14
99592 10342 29105 78532 83018 11639 92015 77204 30914 21912 34519 80835 100000 1
"#
            ),
            r#"42685"#
        );
    }
}
