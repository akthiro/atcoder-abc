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
    let mut i = 1;

    while i * i * i <= N {
        let n = i * i * i;

        let s1 = n.to_string().chars().collect::<Vec<_>>();
        let mut s2 = s1.clone();
        s2.reverse();

        if s1 == s2 {
            result = n;
        }

        i += 1;
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
                r#"345
"#
            ),
            r#"343"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"6
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"123456789012345
"#
            ),
            r#"1334996994331"#
        );
    }
}
