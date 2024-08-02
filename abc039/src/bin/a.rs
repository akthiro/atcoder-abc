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
        C: usize,
    }

    let result = A * B * 2 + B * C * 2 + C * A * 2;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 3 4
"#
            ),
            r#"52"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 4 2
"#
            ),
            r#"52"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100 100 100
"#
            ),
            r#"60000"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"1 1 1
"#
            ),
            r#"6"#
        );
    }
}
