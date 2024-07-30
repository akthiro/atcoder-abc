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
    }

    let mut h = N;

    let s = h % 60;
    h /= 60;

    let m = h % 60;
    h /= 60;

    format!("{:0>2}:{:0>2}:{:0>2}", h, m, s)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3661
"#
            ),
            r#"01:01:01"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"86399
"#
            ),
            r#"23:59:59"#
        );
    }
}
