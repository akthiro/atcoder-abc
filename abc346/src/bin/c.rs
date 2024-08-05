#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashSet,
    io::{self, Read},
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
        K: usize,
        A: [usize; N],
    }

    let mut set = HashSet::new();

    for x in A {
        if x > K {
            continue;
        }

        set.insert(x);
    }

    let mut result = K * (K + 1) / 2;

    for x in set {
        result -= x;
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
                r#"4 5
1 6 3 1
"#
            ),
            r#"11"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 3
346
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 158260522
877914575 24979445 623690081 262703497 24979445 1822804784 1430302156 1161735902 923078537 1189330739
"#
            ),
            r#"12523196466007058"#
        );
    }
}
