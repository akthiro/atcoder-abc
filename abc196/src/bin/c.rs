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

    let mut result = 0;

    for n in 1..10000000 {
        let mut m = n;

        for _ in 0..n.to_string().len() {
            m *= 10;
        }

        if m + n > N {
            break;
        }

        result += 1;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"33
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1333
"#
            ),
            r#"13"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10000000
"#
            ),
            r#"999"#
        );
    }
}
