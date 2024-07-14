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
        X: usize,
        T: usize,
    }

    let result = if N % X == 0 {
        (N / X) * T
    } else {
        ((N / X) + 1) * T
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
                r#"20 12 6
"#
            ),
            r#"12"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1000 1 1000
"#
            ),
            r#"1000000"#
        );
    }
}
