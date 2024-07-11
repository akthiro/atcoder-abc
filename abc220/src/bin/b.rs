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
        K: usize,
        A: usize,
        B: usize,
    }

    let result = to_base10(A, K) * to_base10(B, K);

    format!("{}", result)
}

fn to_base10(n: usize, k: usize) -> usize {
    let mut result = 0;
    let mut n = n;
    let mut exp = 0;

    while n > 0 {
        result += k.pow(exp) * (n % 10);
        n /= 10;
        exp += 1;
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
                r#"2
1011 10100
"#
            ),
            r#"220"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
123 456
"#
            ),
            r#"15642"#
        );
    }
}
