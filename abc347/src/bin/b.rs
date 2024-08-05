#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::{
    collections::HashSet,
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
        S: Chars,
    }

    let mut result = HashSet::new();

    for i in 0..S.len() {
        for j in i..S.len() {
            result.insert(&S[i..=j]);
        }
    }

    format!("{}", result.len())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"yay
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"aababc
"#
            ),
            r#"17"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"abracadabra
"#
            ),
            r#"54"#
        );
    }
}
