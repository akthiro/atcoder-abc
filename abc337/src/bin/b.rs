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

    #[allow(clippy::nonminimal_bool)]
    for i in 0..S.len() - 1 {
        if !((S[i] == 'A' && S[i + 1] == 'A')
            || (S[i] == 'A' && S[i + 1] == 'B')
            || (S[i] == 'A' && S[i + 1] == 'C')
            || (S[i] == 'B' && S[i + 1] == 'B')
            || (S[i] == 'B' && S[i + 1] == 'C')
            || (S[i] == 'C' && S[i + 1] == 'C'))
        {
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
                r#"AAABBBCCCCCCC
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"ACABABCBC
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"A
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"ABBBBBBBBBBBBBCCCCCC
"#
            ),
            r#"Yes"#
        );
    }
}
