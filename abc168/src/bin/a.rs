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

    let n = N % 10;

    match n {
        2 | 4 | 5 | 7 | 9 => "hon".to_string(),
        0 | 1 | 6 | 8 => "pon".to_string(),
        3 => "bon".to_string(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"16
"#
            ),
            r#"pon"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
"#
            ),
            r#"hon"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"183
"#
            ),
            r#"bon"#
        );
    }
}
