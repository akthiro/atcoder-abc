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
        X: [isize; N],
    }

    let mut result = isize::MAX;

    for i in 1..=100 {
        let mut sum = 0;

        for j in X.iter() {
            sum += (j - i) * (j - i);
        }

        result = cmp::min(result, sum);
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
                r#"2
1 4
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
14 14 2 13 56 2 37
"#
            ),
            r#"2354"#
        );
    }
}
