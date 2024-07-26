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
        _N: usize,
        S: String,
    }

    S.find("ABC")
        .map(|n| (n + 1).to_string())
        .unwrap_or("-1".to_string())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"8
ABABCABC
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
ACB
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"20
BBAAABBACAACABCBABAB
"#
            ),
            r#"13"#
        );
    }
}
