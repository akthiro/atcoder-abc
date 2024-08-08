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

    let n = f(N - 1);

    n.to_string()
        .replace('4', "8")
        .replace('3', "6")
        .replace('2', "4")
        .replace('1', "2")
}

fn f(n: usize) -> usize {
    let mut result = 0;
    let mut n = n;
    let mut exp = 0;

    while n > 0 {
        result += 10_usize.pow(exp) * (n % 5);
        n /= 5;
        exp += 1;
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
                r#"8
"#
            ),
            r#"24"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"133
"#
            ),
            r#"2024"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"31415926535
"#
            ),
            r#"2006628868244228"#
        );
    }
}
