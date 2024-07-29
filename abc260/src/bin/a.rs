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

    let a = S[0];
    let b = S[1];
    let c = S[2];

    if a != b && a != c {
        a.to_string()
    } else if b != a && b != c {
        b.to_string()
    } else if c != a && c != b {
        c.to_string()
    } else {
        (-1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"pop
"#
            ),
            r#"o"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"abc
"#
            ),
            r#"a"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"xxx
"#
            ),
            r#"-1"#
        );
    }
}
