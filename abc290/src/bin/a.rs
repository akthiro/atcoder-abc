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
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }

    let mut result = 0;

    for (i, n) in A.iter().enumerate() {
        for m in B.iter() {
            if i + 1 == *m {
                result += n;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 2
10 20 30
1 3
"#
            ),
            r#"40"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 1
1 1 1 100
4
"#
            ),
            r#"100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 4
22 75 26 45 72 81 47 29
4 6 7 8
"#
            ),
            r#"202"#
        );
    }
}
