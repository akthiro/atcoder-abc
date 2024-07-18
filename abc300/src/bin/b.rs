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
        H: usize,
        W: usize,
        A: [Chars; H],
        B: [Chars; H],
    }

    for i in 0..H {
        for j in 0..W {
            let mut ok = true;

            for k in 0..H {
                for l in 0..W {
                    if A[(H + k - i) % H][(W + l - j) % W] != B[k][l] {
                        ok = false;
                    }
                }
            }

            if ok {
                return "Yes".to_string();
            }
        }
    }

    "No".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 3
..#
...
.#.
...
#..
...
.#.
...
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 2
##
##
#.
..
#.
#.
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4 5
#####
.#...
.##..
..##.
...##
#...#
#####
...#.
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"10 30
..........##########..........
..........####....###.....##..
.....##....##......##...#####.
....####...##..#####...##...##
...##..##..##......##..##....#
#.##....##....##...##..##.....
..##....##.##..#####...##...##
..###..###..............##.##.
.#..####..#..............###..
#..........##.................
................#..........##.
######....................####
....###.....##............####
.....##...#####......##....##.
.#####...##...##....####...##.
.....##..##....#...##..##..##.
##...##..##.....#.##....##....
.#####...##...##..##....##.##.
..........##.##...###..###....
...........###...#..####..#...
"#
            ),
            r#"Yes"#
        );
    }
}
