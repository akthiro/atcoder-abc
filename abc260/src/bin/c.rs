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
        X: usize,
        Y: usize,
    }

    let result = sum(N, Color::Red, X, Y);

    format!("{}", result)
}

enum Color {
    Red,
    Blue,
}

fn sum(level: usize, color: Color, x: usize, y: usize) -> usize {
    match (level, color) {
        (1, Color::Red) => 0,
        (1, Color::Blue) => 1,
        (level, Color::Red) => {
            sum(level - 1, Color::Red, x, y) + (sum(level, Color::Blue, x, y) * x)
        }
        (level, Color::Blue) => {
            sum(level - 1, Color::Red, x, y) + (sum(level - 1, Color::Blue, x, y) * y)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 3 4
"#
            ),
            r#"12"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 5 5
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 5 5
"#
            ),
            r#"3942349900"#
        );
    }
}
