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
        _N: usize,
        S: Chars,
    }

    let mut v = vec![];

    for c in S {
        v.push(c);
        v.push(c);
    }

    v.iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"8
beginner
"#
            ),
            r#"bbeeggiinnnneerr"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
aaa
"#
            ),
            r#"aaaaaa"#
        );
    }
}
