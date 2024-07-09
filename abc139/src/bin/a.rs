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
        T: String,
    }

    let mut result = 0;

    let s = S.chars().collect::<Vec<_>>();
    let t = T.chars().collect::<Vec<_>>();

    for i in 0..3 {
        if s[i as usize] == t[i as usize] {
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
                r#"CSS
CSR
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"SSR
SSR
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"RRR
SSS
"#
            ),
            r#"0"#
        );
    }
}
