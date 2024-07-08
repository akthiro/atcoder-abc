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

    if A <= 8 && B <= 8 {
        "Yay!".to_string()
    } else {
        ":(".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 4
"#
            ),
            r#"Yay!"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8 8
"#
            ),
            r#"Yay!"#
        );
    }
}
