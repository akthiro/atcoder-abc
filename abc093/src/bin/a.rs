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

    let chars = S.chars().collect::<Vec<_>>();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for ch in chars {
        match ch {
            'a' => a += 1,
            'b' => b += 1,
            'c' => c += 1,
            _ => unreachable!(),
        }
    }

    if a == 1 && b == 1 && c == 1 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"bac
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"bab
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"abc
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"aaa
"#
            ),
            r#"No"#
        );
    }
}
