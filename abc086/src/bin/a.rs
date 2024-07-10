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
        a: usize,
        b: usize,
    }

    if a * b % 2 == 0 {
        "Even".to_string()
    } else {
        "Odd".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 4
"#
            ),
            r#"Even"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 21
"#
            ),
            r#"Odd"#
        );
    }
}
