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

    for n in 1..=10000 {
        let a = n * 8 / 100;
        let b = n * 10 / 100;

        if a == A && b == B {
            return n.to_string();
        }
    }

    (-1).to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 2
"#
            ),
            r#"25"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8 10
"#
            ),
            r#"100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"19 99
"#
            ),
            r#"-1"#
        );
    }
}
