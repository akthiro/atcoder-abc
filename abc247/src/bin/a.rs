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
        S: Chars,
    }

    let s1 = '0';
    let s2 = S[0];
    let s3 = S[1];
    let s4 = S[2];

    format!("{}{}{}{}", s1, s2, s3, s4)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1011
"#
            ),
            r#"0101"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"0000
"#
            ),
            r#"0000"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1111
"#
            ),
            r#"0111"#
        );
    }
}
