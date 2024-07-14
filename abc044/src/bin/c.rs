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
        A: usize,
        x: [usize; N],
    }

    let mut dp = vec![vec![vec![0_usize; 3000]; 55]; 55];
    dp[0][0][0] = 1;

    for i in 0..N {
        for j in 0..N {
            for k in 0..3000 {
                if dp[i][j][k] == 0 {
                    continue;
                }

                dp[i + 1][j][k] += dp[i][j][k];
                dp[i + 1][j + 1][k + x[i]] += dp[i][j][k];
            }
        }
    }

    let mut result = 0;

    for i in 1..N + 1 {
        result += dp[N][i][i * A];
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
                r#"4 8
7 9 8 9
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 8
6 6 9
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 5
3 6 2 8 7 6 5 9
"#
            ),
            r#"19"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"33 3
3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3 3
"#
            ),
            r#"8589934591"#
        );
    }
}
