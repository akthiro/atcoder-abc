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
    }

    let mut result = vec![];
    let mut ones = vec![];

    for i in 0..60_usize {
        if N >> i & 1 != 0 {
            ones.push(i);
        }
    }

    for i in 0..1_usize << ones.len() {
        let mut n = 0;

        for (j, m) in ones.iter().enumerate() {
            if i >> j & 1 != 0 {
                n += 1_usize << m;
            }
        }

        result.push(n)
    }

    result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"11
"#
            ),
            r#"0
1
2
3
8
9
10
11"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"0
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"576461302059761664
"#
            ),
            r#"0
524288
549755813888
549756338176
576460752303423488
576460752303947776
576461302059237376
576461302059761664"#
        );
    }
}
