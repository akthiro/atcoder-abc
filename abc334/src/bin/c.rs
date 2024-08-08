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
        _N: usize,
        K: usize,
        A: [usize; K],
    }

    let result = if K % 2 == 0 {
        let mut result = 0;

        for i in 0..K / 2 {
            result += A[2 * i + 1] - A[2 * i];
        }

        result
    } else {
        let mut l = vec![0; K / 2 + 1];

        for i in 0..K / 2 {
            l[i + 1] = l[i] + A[2 * i + 1] - A[2 * i];
        }

        let mut r = vec![0; K / 2 + 1];

        for i in 0..K / 2 {
            r[i + 1] = r[i] + A[(K - 1) - (2 * i)] - A[(K - 1) - (2 * i + 1)];
        }

        let mut result = usize::MAX;

        for i in 0..K / 2 + 1 {
            result = result.min(l[i] + r[K / 2 - i]);
        }

        result
    };

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 2
1 3
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 1
2
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
1 2 4 7 8
"#
            ),
            r#"2"#
        );
    }
}
