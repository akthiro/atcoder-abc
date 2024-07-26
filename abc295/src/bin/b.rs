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
        R: usize,
        C: usize,
        B: [Chars; R],
    }

    let mut result = B;

    for i in 0..R {
        for j in 0..C {
            if !result[i][j].is_ascii_digit() {
                continue;
            }

            let n = result[i][j].to_digit(10).unwrap() as isize;

            for (x, rows) in result.iter_mut().enumerate() {
                for (y, col) in rows.iter_mut().enumerate() {
                    if col.is_ascii_digit() {
                        continue;
                    }

                    if (i as isize - x as isize).abs() + (j as isize - y as isize).abs() <= n {
                        *col = '.';
                    }
                }
            }

            result[i][j] = '.';
        }
    }

    result
        .iter()
        .map(|cs| cs.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 4
.1.#
###.
.#2.
#.##
"#
            ),
            r#"...#
#...
....
#..."#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 5
..#.#
###.#
"#
            ),
            r#"..#.#
###.#"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 3
11#
###
"#
            ),
            r#"...
..#"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"4 6
#.#3#.
###.#.
##.###
#1..#.
"#
            ),
            r#"......
#.....
#....#
....#."#
        );
    }
}
