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
        n: usize,
        S: [String; n],
    }

    let mut result = vec![0; 26];

    for (i, ch) in ('a'..='z').enumerate() {
        let mut min = usize::MAX;

        for s in &S {
            min = cmp::min(min, s.chars().filter(|c| *c == ch).count());
        }

        result[i] = cmp::max(result[i], min);
    }

    result
        .iter()
        .enumerate()
        .filter(|(_, n)| **n > 0)
        .map(|(i, n)| format!("{}", char::from_u32(i as u32 + 97).unwrap()).repeat(*n))
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
cbaa
daacc
acacac
"#
            ),
            r#"aac"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
a
aa
b
"#
            ),
            r#""#
        );
    }
}
