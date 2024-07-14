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
        c: char,
    }

    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => "vowel".to_string(),
        _ => "consonant".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"a
"#
            ),
            r#"vowel"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"z
"#
            ),
            r#"consonant"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"s
"#
            ),
            r#"consonant"#
        );
    }
}
