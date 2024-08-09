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

    let mut result = 3;
    let mut n = 1;

    let mut a: usize = 1;
    let mut b: usize = 1;
    let mut c: usize = 1;

    while n < N {
        if a == b && b == c {
            a = 1;
            b = 1;
            c = c * 10 + 1;
        } else if a == b {
            a = 1;
            b = b * 10 + 1;
        } else if a != b {
            a = a * 10 + 1;
        } else {
            unreachable!()
        }

        result = a + b + c;
        n += 1;
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
                r#"5
"#
            ),
            r#"113"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"19
"#
            ),
            r#"2333"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"333
"#
            ),
            r#"112222222233"#
        );
    }
}
