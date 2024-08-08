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
        A: usize,
        B: usize,
        D: usize,
    }

    let mut result = vec![];

    for n in (A..=B).step_by(D) {
        result.push(n);
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
                r#"3 9 2
"#
            ),
            r#"3 5 7 9"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10 10 1
"#
            ),
            r#"10"#
        );
    }
}
