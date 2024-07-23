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
        K: usize,
        S: Chars,
    }

    let mut result = vec![];
    let mut k = K;

    for c in S {
        if c == 'o' && k > 0 {
            result.push("o");
            k -= 1;
        } else {
            result.push("x");
        }
    }

    result.join("")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"10 3
oxxoxooxox
"#
            ),
            r#"oxxoxoxxxx"#
        );
    }
}
