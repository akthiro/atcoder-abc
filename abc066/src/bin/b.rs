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
        S: String,
    }

    for n in 1..S.len() {
        let s = &S[..S.len() - n];

        if s.len() % 2 != 0 {
            continue;
        }

        if s[..s.len() / 2] == s[s.len() / 2..] {
            return s.len().to_string();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"abaababaab
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"xxxx
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"abcabcabcabc
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"akasakaakasakasakaakas
"#
            ),
            r#"14"#
        );
    }
}
