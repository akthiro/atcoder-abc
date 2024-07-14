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
        T: String,
    }

    let chars_s = S.chars().collect::<Vec<_>>();
    let chars_t = T.chars().collect::<Vec<_>>();

    for i in 0..chars_s.len() {
        if chars_s[i] != chars_t[i] {
            return (i + 1).to_string();
        }
    }

    chars_t.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"atcoder
atcorder
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"million
milllion
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"vvwvw
vvvwvw
"#
            ),
            r#"3"#
        );
    }
}
