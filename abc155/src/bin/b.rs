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

    if A.iter().all(|n| {
        if n % 2 == 0 {
            n % 3 == 0 || n % 5 == 0
        } else {
            true
        }
    }) {
        "APPROVED".to_string()
    } else {
        "DENIED".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5
6 7 9 10 31
"#
            ),
            r#"APPROVED"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
28 27 24
"#
            ),
            r#"DENIED"#
        );
    }
}
