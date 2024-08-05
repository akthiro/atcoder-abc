#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        S: Chars,
    }

    let mut v = [0_usize; 26];

    for c in S {
        v[(u32::from(c) - 97) as usize] += 1;
    }

    let mut result = 0;

    for i in 0..26 {
        for j in i + 1..26 {
            result += v[i] * v[j];
        }
    }

    if v.iter().any(|n| *n > 1) {
        result += 1;
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
                r#"abc
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"aaaaa
"#
            ),
            r#"1"#
        );
    }
}
