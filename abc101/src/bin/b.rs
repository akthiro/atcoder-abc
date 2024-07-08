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

    if N % sum_digits(N) == 0 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn sum_digits(n: usize) -> usize {
    let mut result = 0;
    let mut n = n;

    while n > 0 {
        result += n % 10;
        n /= 10;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"12
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"101
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"999999999
"#
            ),
            r#"Yes"#
        );
    }
}
