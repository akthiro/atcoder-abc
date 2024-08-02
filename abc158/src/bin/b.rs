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
        N: usize,
        A: usize,
        B: usize,
    }

    let q = N / (A + B);
    let r = N % (A + B);

    let result = (q * A) + cmp::min(r, A);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"8 3 4
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8 0 4
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6 2 4
"#
            ),
            r#"2"#
        );
    }
}
