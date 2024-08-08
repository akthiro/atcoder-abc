#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashMap,
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
    }

    let mut memo = HashMap::new();
    let result = f(N, &mut memo);

    format!("{}", result)
}

fn f(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n <= 1 {
        return 0;
    }

    if !memo.contains_key(&n) {
        let m = n + f(n / 2, memo) + f((n + 1) / 2, memo);
        memo.insert(n, m);
    }

    *memo.get(&n).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"340
"#
            ),
            r#"2888"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100000000000000000
"#
            ),
            r#"5655884811924144128"#
        );
    }
}
