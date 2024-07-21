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
        C: [Chars; H],
    }

    let mut result = vec![];

    for i in 0..W {
        let mut sum = 0;

        for row in C.iter() {
            if row[i] == '#' {
                sum += 1;
            }
        }

        result.push(sum);
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
                r#"3 4
#..#
.#.#
.#.#
"#
            ),
            r#"1 2 0 3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 7
.......
.......
.......
"#
            ),
            r#"0 0 0 0 0 0 0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 3
.#.
###
.#.
.#.
.##
..#
##.
.##
"#
            ),
            r#"2 7 4"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"5 47
.#..#..#####..#...#..#####..#...#...###...#####
.#.#...#.......#.#...#......##..#..#...#..#....
.##....#####....#....#####..#.#.#..#......#####
.#.#...#........#....#......#..##..#...#..#....
.#..#..#####....#....#####..#...#...###...#####
"#
            ),
            r#"0 5 1 2 2 0 0 5 3 3 3 3 0 0 1 1 3 1 1 0 0 5 3 3 3 3 0 0 5 1 1 1 5 0 0 3 2 2 2 2 0 0 5 3 3 3 3"#
        );
    }
}
