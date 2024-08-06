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

    let a = S[0];
    let b = S[1];
    let c = S[2];

    if a != b {
        if a == c {
            2.to_string()
        } else if b == c {
            1.to_string()
        } else {
            unreachable!()
        }
    } else {
        for (i, c) in S.iter().enumerate().skip(2) {
            if a != *c {
                return (i + 1).to_string();
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"yay
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"egg
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"zzzzzwz
"#
            ),
            r#"6"#
        );
    }
}
