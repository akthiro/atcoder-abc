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
        M: usize,
    }

    let mut result = 0;
    let s = N;
    let mut c = M;

    if s * 2 <= c {
        result += s;
        c -= s * 2;
    } else {
        result += c / 2;
        c %= 2;
    }

    result += c / 4;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1 6
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"12345 678901
"#
            ),
            r#"175897"#
        );
    }
}
