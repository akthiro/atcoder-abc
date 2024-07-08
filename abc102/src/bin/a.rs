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
    }

    if N % 2 == 0 {
        N.to_string()
    } else {
        (N * 2).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
"#
            ),
            r#"10"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"999999999
"#
            ),
            r#"1999999998"#
        );
    }
}
