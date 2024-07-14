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
    let mut chars_t = T.chars().collect::<Vec<_>>();

    let mut done = false;

    for i in 0..chars_s.len() - 1 {
        if chars_s[i] == chars_t[i] {
            continue;
        }

        if !done && chars_s[i] == chars_t[i + 1] && chars_s[i + 1] == chars_t[i] {
            chars_t.swap(i, i + 1);
            done = true;
        } else {
            return "No".to_string();
        }
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"abc
acb
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"aabb
bbaa
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"abcde
abcde
"#
            ),
            r#"Yes"#
        );
    }
}
