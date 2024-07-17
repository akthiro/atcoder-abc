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

    let mut a = A;
    let mut b = B;

    while a > 0 || b > 0 {
        let n = a % 10;
        let m = b % 10;

        if n + m >= 10 {
            return "Hard".to_string();
        }

        a /= 10;
        b /= 10;
    }

    "Easy".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"229 390
"#
            ),
            r#"Hard"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"123456789 9876543210
"#
            ),
            r#"Easy"#
        );
    }
}
