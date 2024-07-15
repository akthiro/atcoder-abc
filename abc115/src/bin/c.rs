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
        h: [usize; N],
    }

    let mut h = h;
    h.sort();

    let mut result = usize::MAX;

    for l in 0..N - K + 1 {
        let r = l + K - 1;

        result = cmp::min(result, h[r] - h[l]);
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
                r#"5 3
10
15
11
14
12
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 3
5
7
5
7
7
"#
            ),
            r#"0"#
        );
    }
}
