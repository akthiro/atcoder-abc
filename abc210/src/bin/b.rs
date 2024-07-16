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
        _N: usize,
        S: String,
    }

    if S.chars().enumerate().find(|(_, c)| *c == '1').unwrap().0 % 2 == 0 {
        "Takahashi".to_string()
    } else {
        "Aoki".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5
00101
"#
            ),
            r#"Takahashi"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
010
"#
            ),
            r#"Aoki"#
        );
    }
}
