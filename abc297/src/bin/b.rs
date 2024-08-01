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
    }

    let mut is_odd = false;
    let mut is_even = false;

    let mut r = vec![];
    let mut k = 0;

    for (i, c) in S.iter().enumerate() {
        if *c == 'B' {
            if i % 2 != 0 {
                is_odd = true;
            } else {
                is_even = true;
            }
        }

        if *c == 'R' {
            r.push(i);
        }

        if *c == 'K' {
            k = i;
        }
    }

    if !(is_odd && is_even) {
        return "No".to_string();
    }

    if !(r[0] < k && k < r[1]) {
        return "No".to_string();
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"RNBQKBNR
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"KRRBBNNQ
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"BRKRBQNN
"#
            ),
            r#"No"#
        );
    }
}
