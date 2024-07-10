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
        H: usize,
        W: usize,
        h: usize,
        w: usize,
    }

    let total = H * W;
    let checked = h * W + w * H - h * w;

    let result = total - checked;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 2
2 1
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 5
2 3
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 4
2 4
"#
            ),
            r#"0"#
        );
    }
}
