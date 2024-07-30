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
        T: Chars,
    }

    for i in 1..S.len() {
        for j in 0..i {
            let mut s = vec![];

            for k in (j..S.len()).step_by(i) {
                s.push(S[k]);
            }

            if s == T {
                return "Yes".to_string();
            }
        }
    }

    "No".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"atcoder toe
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"beginner r
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"verticalreading agh
"#
            ),
            r#"No"#
        );
    }
}
