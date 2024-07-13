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
        M: usize,
    }

    let mut n = N;
    let mut m = M;

    if n > m {
        (n, m) = (m, n);
    }

    if n == 1 {
        if m == 1 {
            format!("{}", 1)
        } else {
            format!("{}", m - 2)
        }
    } else {
        format!("{}", (n - 2) * (m - 2))
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 2
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 7
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"314 1592
"#
            ),
            r#"496080"#
        );
    }
}
