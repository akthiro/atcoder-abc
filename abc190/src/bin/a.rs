#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp::Ordering,
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
        A: usize,
        B: usize,
        C: usize,
    }

    match A.cmp(&B) {
        Ordering::Greater => "Takahashi".to_string(),
        Ordering::Less => "Aoki".to_string(),
        Ordering::Equal => {
            if C == 0 {
                "Aoki".to_string()
            } else {
                "Takahashi".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 1 0
"#
            ),
            r#"Takahashi"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 2 0
"#
            ),
            r#"Aoki"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 2 1
"#
            ),
            r#"Takahashi"#
        );
    }
}
