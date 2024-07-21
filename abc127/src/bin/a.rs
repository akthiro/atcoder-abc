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
        A: usize,
        B: usize,
    }

    let result = if A >= 13 {
        B
    } else if (6..=12).contains(&A) {
        B / 2
    } else {
        0
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
                r#"30 100
"#
            ),
            r#"100"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"12 100
"#
            ),
            r#"50"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"0 100
"#
            ),
            r#"0"#
        );
    }
}
