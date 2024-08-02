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
        M: usize,
    }

    let mut n = vec![2_usize; N];
    let mut m = vec![1_usize; M];
    n.append(&mut m);

    let mut result = 0;

    for i in 0..N + M {
        for j in i + 1..N + M {
            if (n[i] + n[j]) % 2 == 0 {
                result += 1;
            }
        }
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
                r#"2 1
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 3
"#
            ),
            r#"9"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1 1
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"13 3
"#
            ),
            r#"81"#
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            solve(
                r#"0 3
"#
            ),
            r#"3"#
        );
    }
}
