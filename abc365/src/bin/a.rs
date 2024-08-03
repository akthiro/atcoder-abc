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
        Y: usize,
    }

    if Y % 4 != 0 {
        return 365.to_string();
    }

    if Y % 4 == 0 && Y % 100 != 0 {
        return 366.to_string();
    }

    if Y % 100 == 0 && Y % 400 != 0 {
        return 365.to_string();
    }

    if Y % 400 == 0 {
        return 366.to_string();
    }

    365.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2023
"#
            ),
            r#"365"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1992
"#
            ),
            r#"366"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1800
"#
            ),
            r#"365"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"1600
"#
            ),
            r#"366"#
        );
    }
}
