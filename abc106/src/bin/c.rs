#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        S: Chars,
        K: usize,
    }

    let mut one = 0;

    for c in S.iter() {
        if *c == '1' {
            one += 1;
        } else {
            break;
        }
    }

    if one >= K {
        1.to_string()
    } else {
        format!("{}", S[one])
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1214
4
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
157
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"299792458
9460730472580800
"#
            ),
            r#"2"#
        );
    }
}
