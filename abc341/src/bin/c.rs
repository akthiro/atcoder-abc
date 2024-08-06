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
        _N: usize,
        T: Chars,
        S: [Chars; H],
    }

    let mut result = 0;

    for i in 0..H {
        'grid: for j in 0..W {
            if S[i][j] == '#' {
                continue;
            }

            let mut i = i;
            let mut j = j;

            for c in T.iter() {
                match c {
                    'L' => {
                        if S[i][j - 1] == '#' {
                            continue 'grid;
                        }

                        j -= 1;
                    }
                    'R' => {
                        if S[i][j + 1] == '#' {
                            continue 'grid;
                        }

                        j += 1;
                    }
                    'U' => {
                        if S[i - 1][j] == '#' {
                            continue 'grid;
                        }

                        i -= 1;
                    }
                    'D' => {
                        if S[i + 1][j] == '#' {
                            continue 'grid;
                        }

                        i += 1;
                    }
                    _ => unreachable!(),
                }
            }

            result += 1;
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
                r#"6 7 5
LULDR
#######
#...#.#
##...##
#.#...#
#...#.#
#######
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"13 16 9
ULURDLURD
################
##..##.#..####.#
###.#..#.....#.#
#..##..#####.###
#...#..#......##
###.##.#..#....#
##.#####....##.#
###.###.#.#.#..#
######.....##..#
#...#.#.######.#
##..###..#..#.##
#...#.#.#...#..#
################
"#
            ),
            r#"6"#
        );
    }
}
