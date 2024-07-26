#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
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
        M: usize,
        S: Chars,
        T: Chars,
    }

    let mut is_prefix = true;

    for i in 0..N {
        if S[i] != T[i] {
            is_prefix = false;
            break;
        }
    }

    let mut is_suffix = true;

    for i in 0..N {
        if S[i] != T[M - N + i] {
            is_suffix = false;
            break;
        }
    }

    match (is_prefix, is_suffix) {
        (true, true) => 0.to_string(),
        (true, false) => 1.to_string(),
        (false, true) => 2.to_string(),
        (false, false) => 3.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 7
abc
abcdefg
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 4
abc
aabc
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 3
abc
xyz
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"3 3
aaa
aaa
"#
            ),
            r#"0"#
        );
    }
}
