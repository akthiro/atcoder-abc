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

    let mut result = 0;

    for n in A..=B {
        if n == reverse(n) {
            result += 1;
        }
    }

    format!("{}", result)
}

fn reverse(n: usize) -> usize {
    let mut result = 0;
    let mut n = n;

    while n > 0 {
        result = (result * 10) + (n % 10);
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
                r#"11009 11332
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"31415 92653
"#
            ),
            r#"612"#
        );
    }
}
