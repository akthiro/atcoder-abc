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

    let mut n = N;
    let mut result = vec![];

    while n > 0 {
        if n % 2 == 1 {
            result.push("A");
            n -= 1;
        } else {
            result.push("B");
            n /= 2;
        }
    }

    result.reverse();

    result.join("")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5
"#
            ),
            r#"ABBA"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"14
"#
            ),
            r#"ABABAB"#
        );
    }
}
