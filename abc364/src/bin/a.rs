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
        N: usize,
        S: [String; N],
    }

    if N == 1 {
        return "Yes".to_string();
    }

    for i in 0..N - 2 {
        if S[i] == "sweet" && S[i + 1] == "sweet" {
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
                r#"5
salty
sweet
salty
salty
sweet
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
sweet
salty
sweet
sweet
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
salty
sweet
sweet
salty
sweet
sweet
"#
            ),
            r#"No"#
        );
    }
}
