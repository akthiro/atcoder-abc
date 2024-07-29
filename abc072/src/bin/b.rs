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
        s: Chars,
    }

    s.iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| c)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"atcoder
"#
            ),
            r#"acdr"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"aaaa
"#
            ),
            r#"aa"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"z
"#
            ),
            r#"z"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"fukuokayamaguchi
"#
            ),
            r#"fkoaaauh"#
        );
    }
}
