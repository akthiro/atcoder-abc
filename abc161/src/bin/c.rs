#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: isize,
        K: isize,
    }

    let result = cmp::min(N % K, K - N % K);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7 4
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 6
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1000000000000000000 1
"#
            ),
            r#"0"#
        );
    }
}
