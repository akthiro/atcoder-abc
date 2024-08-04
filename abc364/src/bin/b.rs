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
        H: isize,
        W: isize,
        S: (isize, isize),
        C: [Chars; H],
        X: Chars,
    }

    let mut i = S.0 - 1;
    let mut j = S.1 - 1;

    for o in X {
        match o {
            'L' => {
                if j > 0 && C[i as usize][j as usize - 1] != '#' {
                    j -= 1;
                }
            }
            'R' => {
                if j < W - 1 && C[i as usize][j as usize + 1] != '#' {
                    j += 1;
                }
            }
            'U' => {
                if i > 0 && C[i as usize - 1][j as usize] != '#' {
                    i -= 1;
                }
            }
            'D' => {
                if i < H - 1 && C[i as usize + 1][j as usize] != '#' {
                    i += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    format!("{} {}", i + 1, j + 1)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 3
2 1
.#.
...
ULDRU
"#
            ),
            r#"2 2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 4
4 2
....
.#..
...#
....
DUUUURULRD
"#
            ),
            r#"2 4"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6 6
1 1
.#####
######
######
######
######
######
RURLDLULLRULRDL
"#
            ),
            r#"1 1"#
        );
    }
}
