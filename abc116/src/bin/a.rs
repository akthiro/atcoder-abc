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
        AB: usize,
        BC: usize,
        CA: usize,
    }

    let mut l = [AB, BC, CA];
    l.sort();

    let result = l[0] * l[1] / 2;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 4 5
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 12 13
"#
            ),
            r#"30"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"45 28 53
"#
            ),
            r#"630"#
        );
    }
}
