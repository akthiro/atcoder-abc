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

    let s = S.clone();
    let mut r = s.clone();
    r.reverse();

    let s1 = S.iter().take(S.len() / 2).collect::<Vec<_>>();
    let mut r1 = s1.clone();
    r1.reverse();

    let s2 = S.iter().skip((S.len() / 2) + 1).collect::<Vec<_>>();
    let mut r2 = s2.clone();
    r2.reverse();

    if s == r && s1 == r1 && s2 == r2 {
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
                r#"akasaka
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"level
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"atcoder
"#
            ),
            r#"No"#
        );
    }
}
