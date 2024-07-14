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

    let words = ["dream", "dreamer", "erase", "eraser"];

    let mut dp = vec![false; S.len() + 1];
    dp[0] = true;

    for i in 1..S.len() + 1 {
        for word in words {
            if i >= word.len() && dp[i - word.len()] && &S[i - word.len()..i] == word {
                dp[i] = true;
            }
        }
    }

    if dp[S.len()] {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"erasedream
"#
            ),
            r#"YES"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"dreameraser
"#
            ),
            r#"YES"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"dreamerer
"#
            ),
            r#"NO"#
        );
    }
}
