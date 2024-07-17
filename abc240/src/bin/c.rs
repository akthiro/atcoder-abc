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
        X: usize,
        ab: [(usize, usize); N],
    }

    let mut dp = vec![vec![false; X + 1]; N + 1];
    dp[0][0] = true;

    for i in 0..N {
        for j in 0..X + 1 {
            if dp[i][j] {
                if j + ab[i].0 <= X {
                    dp[i + 1][j + ab[i].0] = true;
                }

                if j + ab[i].1 <= X {
                    dp[i + 1][j + ab[i].1] = true;
                }
            }
        }
    }

    if dp[N][X] {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 10
3 6
4 5
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 10
10 100
10 100
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4 12
1 8
5 7
3 4
2 6
"#
            ),
            r#"Yes"#
        );
    }
}
