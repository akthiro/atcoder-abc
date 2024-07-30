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

    "x".repeat(S.len())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"sardine
"#
            ),
            r#"xxxxxxx"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"xxxx
"#
            ),
            r#"xxxx"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"gone
"#
            ),
            r#"xxxx"#
        );
    }
}
