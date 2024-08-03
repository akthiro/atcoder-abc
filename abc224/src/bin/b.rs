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
        H: usize,
        W: usize,
        A: [[usize; W]; H],
    }

    for i1 in 0..H {
        for i2 in i1 + 1..H {
            for j1 in 0..W {
                for j2 in j1 + 1..W {
                    if A[i1][j1] + A[i2][j2] > A[i2][j1] + A[i1][j2] {
                        return "No".to_string();
                    }
                }
            }
        }
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 3
2 1 4
3 1 3
6 4 1
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 4
4 3 2 1
5 6 7 8
"#
            ),
            r#"No"#
        );
    }
}
