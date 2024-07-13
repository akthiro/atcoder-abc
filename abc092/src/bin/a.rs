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
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }

    let result = cmp::min(A, B) + cmp::min(C, D);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"600
300
220
420
"#
            ),
            r#"520"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"555
555
400
200
"#
            ),
            r#"755"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"549
817
715
603
"#
            ),
            r#"1152"#
        );
    }
}
