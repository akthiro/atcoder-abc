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
        X: usize,
    }

    let r = X % 100;

    if r == 0 {
        100.to_string()
    } else {
        (100 - r).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"140
"#
            ),
            r#"60"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1000
"#
            ),
            r#"100"#
        );
    }
}
