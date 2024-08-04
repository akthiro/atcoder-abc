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
    }

    match (A, B) {
        (1, 2) => 3.to_string(),
        (1, 3) => 2.to_string(),
        (2, 1) => 3.to_string(),
        (2, 3) => 1.to_string(),
        (3, 1) => 2.to_string(),
        (3, 2) => 1.to_string(),
        _ => (-1).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1 2
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 1
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 1
"#
            ),
            r#"2"#
        );
    }
}
