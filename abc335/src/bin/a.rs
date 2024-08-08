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

    let mut s = S;
    let i = s.len() - 1;
    s[i] = '4';

    s.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"hello2023
"#
            ),
            r#"hello2024"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"worldtourfinals2023
"#
            ),
            r#"worldtourfinals2024"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2023
"#
            ),
            r#"2024"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"20232023
"#
            ),
            r#"20232024"#
        );
    }
}
