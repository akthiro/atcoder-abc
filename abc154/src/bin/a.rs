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
        S: String,
        _T: String,
        A: usize,
        B: usize,
        U: String,
    }

    if S == U {
        format!("{} {}", A - 1, B)
    } else {
        format!("{} {}", A, B - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"red blue
3 4
red
"#
            ),
            r#"2 4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"red blue
5 5
blue
"#
            ),
            r#"5 4"#
        );
    }
}
