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
        R: usize,
        G: usize,
        B: usize,
        C: String,
    }

    match C.as_str() {
        "Red" => cmp::min(G, B).to_string(),
        "Green" => cmp::min(R, B).to_string(),
        "Blue" => cmp::min(R, G).to_string(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"20 30 10
Blue
"#
            ),
            r#"20"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"100 100 100
Red
"#
            ),
            r#"100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"37 39 93
Blue
"#
            ),
            r#"37"#
        );
    }
}
