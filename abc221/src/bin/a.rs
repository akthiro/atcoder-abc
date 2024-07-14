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
        A: u32,
        B: u32,
    }

    let result = 32_usize.pow(A - B);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"6 4
"#
            ),
            r#"1024"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 5
"#
            ),
            r#"1"#
        );
    }
}
