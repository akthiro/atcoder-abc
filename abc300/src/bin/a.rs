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
        B: usize,
        C: [usize; N],
    }

    (C.iter().enumerate().find(|(_, n)| **n == A + B).unwrap().0 + 1).to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 125 175
200 300 400
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 1 1
2
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5 123 456
135 246 357 468 579
"#
            ),
            r#"5"#
        );
    }
}
