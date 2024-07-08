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
        D: u32,
        N: usize,
    }

    let result = if N == 100 {
        100_usize.pow(D) * 101
    } else {
        100_usize.pow(D) * N
    };

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"0 5
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 11
"#
            ),
            r#"1100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 85
"#
            ),
            r#"850000"#
        );
    }
}
