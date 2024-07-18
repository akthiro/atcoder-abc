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
        A: [String; N],
    }

    let a: Vec<Vec<usize>> = A
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut result = a.clone();

    for i in 0..N {
        for j in 0..N {
            if i == 0 {
                if j == 0 {
                    result[i][j] = a[i + 1][j];
                } else {
                    result[i][j] = a[i][j - 1];
                }
            } else if i == N - 1 {
                if j == N - 1 {
                    result[i][j] = a[i - 1][j];
                } else {
                    result[i][j] = a[i][j + 1];
                }
            } else {
                if j == 0 {
                    result[i][j] = a[i + 1][j];
                }
                if j == N - 1 {
                    result[i][j] = a[i - 1][j];
                }
            }
        }
    }

    result
        .iter()
        .map(|l| l.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(""))
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
                r#"4
0101
1101
1111
0000
"#
            ),
            r#"1010
1101
0111
0001"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
11
11
"#
            ),
            r#"11
11"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5
01010
01001
10110
00110
01010
"#
            ),
            r#"00101
11000
00111
00110
10100"#
        );
    }
}
