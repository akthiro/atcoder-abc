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
        a: [usize; N],
    }

    let mut result = 0;

    let mut a = a;

    for n in a.iter_mut() {
        while *n % 2 == 0 {
            *n /= 2;
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
                r#"3
5 2 4
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
631 577 243 199
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10
2184 2126 1721 1800 1024 2528 3360 1945 1280 1776
"#
            ),
            r#"39"#
        );
    }
}
