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
    }

    let result = if ((1..=3).contains(&A) && (1..=3).contains(&B))
        || ((4..=6).contains(&A) && (4..=6).contains(&B))
        || ((7..=9).contains(&A) && (7..=9).contains(&B))
    {
        A == B - 1
    } else {
        false
    };

    if result {
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
                r#"7 8
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 9
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 4
"#
            ),
            r#"No"#
        );
    }
}
