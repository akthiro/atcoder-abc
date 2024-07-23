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
        A: usize,
        B: usize,
    }

    let max = 2 * A + 100;

    if B < max {
        (max - B).to_string()
    } else {
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"200 300
"#
            ),
            r#"200"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10000 0
"#
            ),
            r#"20100"#
        );
    }
}
