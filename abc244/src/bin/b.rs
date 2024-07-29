#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        _N: usize,
        T: Chars,
    }

    let dx = [1, 0, -1, 0];
    let dy = [0, -1, 0, 1];

    let mut x = 0;
    let mut y = 0;
    let mut dir = 4;

    for c in T {
        match c {
            'S' => {
                let i = dir % 4;
                x += dx[i];
                y += dy[i];
            }
            'R' => {
                dir += 1;
            }
            _ => unreachable!(),
        }
    }

    format!("{} {}", x, y)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
SSRS
"#
            ),
            r#"2 -1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"20
SRSRSSRSSSRSRRRRRSRR
"#
            ),
            r#"0 1"#
        );
    }
}
