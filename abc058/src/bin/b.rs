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
        O: String,
        E: String,
    }

    let o_chars = O.chars().collect::<Vec<_>>();
    let e_chars = E.chars().collect::<Vec<_>>();

    let mut result = vec![];

    for i in 0..e_chars.len() {
        result.push(o_chars[i]);
        result.push(e_chars[i]);
    }

    if o_chars.len() > e_chars.len() {
        result.push(o_chars[o_chars.len() - 1]);
    }

    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"xyz
abc
"#
            ),
            r#"xaybzc"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"atcoderbeginnercontest
atcoderregularcontest
"#
            ),
            r#"aattccooddeerrbreeggiunlnaerrccoonntteesstt"#
        );
    }
}
