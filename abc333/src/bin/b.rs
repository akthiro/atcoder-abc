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

    let s = match (&S[0], &S[1]) {
        ('A', 'E' | 'B') => 1,
        ('B', 'A' | 'C') => 1,
        ('C', 'B' | 'D') => 1,
        ('D', 'C' | 'E') => 1,
        ('E', 'D' | 'A') => 1,
        _ => 2,
    };

    let t = match (&T[0], &T[1]) {
        ('A', 'E' | 'B') => 1,
        ('B', 'A' | 'C') => 1,
        ('C', 'B' | 'D') => 1,
        ('D', 'C' | 'E') => 1,
        ('E', 'D' | 'A') => 1,
        _ => 2,
    };

    if s == t {
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
                r#"AC
EC
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"DA
EA
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"BD
BD
"#
            ),
            r#"Yes"#
        );
    }
}
