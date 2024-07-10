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
        A: [usize; N],
    }

    let mut a = A.iter().enumerate().collect::<Vec<_>>();
    a.sort_by(|a, b| a.1.cmp(b.1));

    a.iter()
        .map(|(i, _)| format!("{}", i + 1))
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
                r#"3
2 3 1
"#
            ),
            r#"3 1 2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
1 2 3 4 5
"#
            ),
            r#"1 2 3 4 5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8
8 2 7 3 4 5 6 1
"#
            ),
            r#"8 2 4 5 6 7 3 1"#
        );
    }
}
