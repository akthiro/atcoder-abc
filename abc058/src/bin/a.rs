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
        a: isize,
        b: isize,
        c: isize,
    }

    if b - a == c - b {
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
                r#"2 4 6
"#
            ),
            r#"YES"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 5 6
"#
            ),
            r#"NO"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 2 1
"#
            ),
            r#"YES"#
        );
    }
}
