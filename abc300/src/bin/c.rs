#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::{
    cmp,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        H: isize,
        W: isize,
        C: [Chars; H],
    }

    let mut result = vec![0_isize; cmp::min(H, W) as usize];

    for i in 0..H {
        for j in 0..W {
            if C[i as usize][j as usize] == '#' {
                let n = {
                    let mut n = 0;

                    loop {
                        let m = n + 1;

                        if !(i + m < H
                            && i - m >= 0
                            && j + m < W
                            && j - m >= 0
                            && C[(i + m) as usize][(j + m) as usize] == '#'
                            && C[(i + m) as usize][(j - m) as usize] == '#'
                            && C[(i - m) as usize][(j + m) as usize] == '#'
                            && C[(i - m) as usize][(j - m) as usize] == '#')
                        {
                            break;
                        }

                        n += 1;
                    }

                    n
                };

                if n <= 0 {
                    continue;
                }

                result[n as usize - 1] += 1;
            }
        }
    }

    result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 9
#.#.#...#
.#...#.#.
#.#...#..
.....#.#.
....#...#
"#
            ),
            r#"1 1 0 0 0"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 3
...
...
...
"#
            ),
            r#"0 0 0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 16
#.#.....#.#..#.#
.#.......#....#.
#.#.....#.#..#.#
"#
            ),
            r#"3 0 0"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"15 20
#.#..#.............#
.#....#....#.#....#.
#.#....#....#....#..
........#..#.#..#...
#.....#..#.....#....
.#...#....#...#..#.#
..#.#......#.#....#.
...#........#....#.#
..#.#......#.#......
.#...#....#...#.....
#.....#..#.....#....
........#.......#...
#.#....#....#.#..#..
.#....#......#....#.
#.#..#......#.#....#
"#
            ),
            r#"5 0 1 0 0 0 1 0 0 0 0 0 0 0 0"#
        );
    }
}
