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
        A: [usize; N],
    }

    let mut result = vec![];

    for i in 0..N - 1 {
        result.push(A[i] * A[i + 1]);
    }

    result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
3 4 6
"#
            ),
            r#"12 24"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
22 75 26 45 72
"#
            ),
            r#"1650 1950 1170 3240"#
        );
    }
}
