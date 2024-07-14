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
    }

    let mut ns = N
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    ns.sort_by(|a, b| b.cmp(a));

    let mut result = 0;

    for i in 0..(1 << ns.len()) {
        let mut l = 0;
        let mut r = 0;

        for (j, n) in ns.iter().enumerate() {
            if i & (1 << j) != 0 {
                l = l * 10 + n;
            } else {
                r = r * 10 + n;
            }
        }

        result = cmp::max(result, l * r);
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
                r#"123
"#
            ),
            r#"63"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1010
"#
            ),
            r#"100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"998244353
"#
            ),
            r#"939337176"#
        );
    }
}
