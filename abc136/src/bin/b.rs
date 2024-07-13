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
        N: usize,
    }

    let mut result = 0;

    for n in 1..=N {
        if n.to_string().chars().collect::<Vec<_>>().len() % 2 != 0 {
            result += 1;
        }
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"11
"#
            ),
            r#"9"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"136
"#
            ),
            r#"46"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100000
"#
            ),
            r#"90909"#
        );
    }
}
