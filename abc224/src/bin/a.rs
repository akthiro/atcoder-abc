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
        S: String,
    }

    if S.ends_with("er") {
        "er".to_string()
    } else if S.ends_with("ist") {
        "ist".to_string()
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"atcoder
"#
            ),
            r#"er"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"tourist
"#
            ),
            r#"ist"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"er
"#
            ),
            r#"er"#
        );
    }
}
