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

    let mut lower = 0;
    let mut upper = 0;

    for c in S.chars() {
        if c.is_ascii_lowercase() {
            lower += 1;
        } else {
            upper += 1;
        }
    }

    if lower < upper {
        S.to_uppercase()
    } else {
        S.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"AtCoder
"#
            ),
            r#"atcoder"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"SunTORY
"#
            ),
            r#"SUNTORY"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"a
"#
            ),
            r#"a"#
        );
    }
}
