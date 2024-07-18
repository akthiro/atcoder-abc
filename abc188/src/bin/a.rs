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
        X: usize,
        Y: usize,
    }

    if cmp::min(X, Y) + 3 > cmp::max(X, Y) {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 5
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"16 2
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"12 15
"#
            ),
            r#"No"#
        );
    }
}
