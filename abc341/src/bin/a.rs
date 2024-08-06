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

    let mut result = vec![1];

    for _ in 0..N {
        result.push(0);
        result.push(1);
    }

    result.iter().map(ToString::to_string).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
"#
            ),
            r#"101010101"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1
"#
            ),
            r#"101"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10
"#
            ),
            r#"101010101010101010101"#
        );
    }
}
