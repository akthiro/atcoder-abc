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
        A: usize,
        B: usize,
    }

    let mut s = S;
    s.swap(A - 1, B - 1);

    s.iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"chokudai
3 5
"#
            ),
            r#"chukodai"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"aa
1 2
"#
            ),
            r#"aa"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"aaaabbbb
1 8
"#
            ),
            r#"baaabbba"#
        );
    }
}
