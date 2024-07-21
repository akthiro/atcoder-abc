#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    io::{self, Read},
    mem,
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: usize,
        X: isize,
        x: [isize; N],
    }

    let x = x.iter().map(|n| n - X).collect::<Vec<_>>();

    let mut result = x[0].abs();

    for n in x.iter() {
        result = gcd(result, n.abs());
    }

    format!("{}", result)
}

fn gcd(n: isize, m: isize) -> isize {
    let mut n = n;
    let mut m = m;

    while m != 0 {
        if m < n {
            mem::swap(&mut m, &mut n);
        }
        m %= n;
    }

    n
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 3
1 7 11
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 81
33 105 57
"#
            ),
            r#"24"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1 1
1000000000
"#
            ),
            r#"999999999"#
        );
    }
}
