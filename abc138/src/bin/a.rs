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
        a: usize,
        s: String,
    }

    if a >= 3200 {
        s
    } else {
        "red".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3200
pink
"#
            ),
            r#"pink"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3199
pink
"#
            ),
            r#"red"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4049
red
"#
            ),
            r#"red"#
        );
    }
}
