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
        C: usize,
    }

    let mut result = 0;

    while result <= B {
        if result >= A {
            return format!("{}", result);
        }

        result += C;
    }

    format!("{}", -1)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"123 456 100
"#
            ),
            r#"200"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"630 940 314
"#
            ),
            r#"-1"#
        );
    }
}
