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
        A: isize,
        B: isize,
        C: isize,
    }

    let result = cmp::max(C - (A - B), 0);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"6 4 3
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8 3 9
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"12 3 7
"#
            ),
            r#"0"#
        );
    }
}
