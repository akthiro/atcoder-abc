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
        X: isize,
    }

    let result = if (X + 9) < 0 && (X + 9) % 10 != 0 {
        (X + 9) / 10 - 1
    } else {
        (X + 9) / 10
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
                r#"27
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"-13
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"40
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"-20
"#
            ),
            r#"-2"#
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            solve(
                r#"123456789123456789
"#
            ),
            r#"12345678912345679"#
        );
    }
}
