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

    let result = (800 * N) - ((N / 15) * 200);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"20
"#
            ),
            r#"15800"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"60
"#
            ),
            r#"47200"#
        );
    }
}
