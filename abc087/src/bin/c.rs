#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
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
        N: usize,
        A: [[usize; N]; 2],
    }

    let mut result = 0;

    for i in 0..N {
        let mut sum = 0;

        for j in 0..=i {
            sum += A[0][j];
        }

        for j in i..N {
            sum += A[1][j];
        }

        result = cmp::max(result, sum);
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
                r#"5
3 2 2 4 1
1 2 2 2 1
"#
            ),
            r#"14"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
1 1 1 1
1 1 1 1
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"7
3 3 4 5 4 5 3
5 3 4 4 2 3 2
"#
            ),
            r#"29"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"1
2
3
"#
            ),
            r#"5"#
        );
    }
}
