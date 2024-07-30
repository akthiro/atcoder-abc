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

    let n = N % 10;
    let m = N / 10;

    match n {
        0..=2 => (m * 10).to_string(),
        3..=7 => (m * 10 + 5).to_string(),
        8..=9 => (m * 11).to_string(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"53
"#
            ),
            r#"55"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"21
"#
            ),
            r#"20"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100
"#
            ),
            r#"100"#
        );
    }
}
