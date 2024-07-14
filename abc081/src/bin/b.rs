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
        A: [usize; N],
    }

    A.iter().map(|n| div_cnt(*n)).min().unwrap().to_string()
}

fn div_cnt(n: usize) -> usize {
    let mut result = 0;
    let mut n = n;

    while n % 2 == 0 {
        result += 1;
        n /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
8 12 40
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
5 6 8 10
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
382253568 723152896 37802240 379425024 404894720 471526144
"#
            ),
            r#"8"#
        );
    }
}
