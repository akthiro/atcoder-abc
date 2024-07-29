#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
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
        X: isize,
        t: isize,
    }

    let result = cmp::max(X - t, 0);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"100 17
"#
            ),
            r#"83"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"48 58
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1000000000 1000000000
"#
            ),
            r#"0"#
        );
    }
}
