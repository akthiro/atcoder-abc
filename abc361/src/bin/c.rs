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
        K: usize,
        A: [usize; N],
    }

    let mut a = A;
    a.sort();

    let mut result = usize::MAX;

    let mut i = 0;
    let j = N - K - 1;

    while i + j < N {
        result = cmp::min(result, a[i + j] - a[i]);
        i += 1;
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
                r#"5 2
3 1 5 4 9
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"6 5
1 1 1 1 1 1
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 3
31 43 26 6 18 36 22 13
"#
            ),
            r#"18"#
        );
    }
}
