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
        T: Chars,
    }

    let mut result = vec![];

    let mut start = 0;

    for s in S {
        for (i, t) in T.iter().enumerate().skip(start) {
            if s == *t {
                result.push(i + 1);
                start = i + 1;
                break;
            }
        }
    }

    result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"abc
axbxyc
"#
            ),
            r#"1 3 6"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"aaaa
bbbbaaaa
"#
            ),
            r#"5 6 7 8"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"atcoder
atcoder
"#
            ),
            r#"1 2 3 4 5 6 7"#
        );
    }
}
