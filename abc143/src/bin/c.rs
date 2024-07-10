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

    let mut result = 0;

    let chars = S.chars().collect::<Vec<_>>();

    let mut tmp = ' ';

    for c in chars {
        if c != tmp {
            result += 1;
            tmp = c;
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
                r#"10
aabbbbaaca
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
aaaaa
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"20
xxzaffeeeeddfkkkkllq
"#
            ),
            r#"10"#
        );
    }
}
