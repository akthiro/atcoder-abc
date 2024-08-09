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
        S: usize,
        M: usize,
        L: usize,
    }

    let mut result = usize::MAX;

    for s in 0..=100 {
        for m in 0..=100 {
            for l in 0..=100 {
                if s * 6 + m * 8 + l * 12 >= N {
                    result = result.min(s * S + m * M + l * L);
                }
            }
        }
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"16 120 150 200
"#
            ),
            r#"300"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10 100 50 10
"#
            ),
            r#"10"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"99 600 800 1200
"#
            ),
            r#"10000"#
        );
    }
}
