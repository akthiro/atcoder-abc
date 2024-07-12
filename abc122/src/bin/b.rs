#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        S: String,
    }

    let chars = S.chars().collect::<Vec<_>>();

    let mut result = 0;

    for start in 0..chars.len() {
        if chars[start] != 'A' && chars[start] != 'C' && chars[start] != 'G' && chars[start] != 'T'
        {
            continue;
        }

        let mut end = start;

        while end < chars.len()
            && (chars[end] == 'A' || chars[end] == 'C' || chars[end] == 'G' || chars[end] == 'T')
        {
            end += 1;
        }

        result = cmp::max(result, end - start);
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
                r#"ATCODER
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"HATAGAYA
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"SHINJUKU
"#
            ),
            r#"0"#
        );
    }
}
