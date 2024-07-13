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
        C: usize,
        X: usize,
    }

    let mut result = 0;

    for i in 0..=A {
        for j in 0..=B {
            for k in 0..=C {
                if 500 * i + 100 * j + 50 * k == X {
                    result += 1;
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
                r#"2
2
2
100
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
1
0
150
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"30
40
50
6000
"#
            ),
            r#"213"#
        );
    }
}
