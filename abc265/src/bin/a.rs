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
        X: usize,
        Y: usize,
        N: usize,
    }

    if 3 * X <= Y {
        (N * X).to_string()
    } else {
        let x = N % 3;
        let y = N / 3;
        (x * X + y * Y).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"10 25 10
"#
            ),
            r#"85"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10 40 10
"#
            ),
            r#"100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100 100 2
"#
            ),
            r#"200"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"100 100 100
"#
            ),
            r#"3400"#
        );
    }
}
