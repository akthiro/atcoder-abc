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
        A: [usize; N],
    }

    let mut m = HashMap::new();

    for (i, x) in A.iter().enumerate() {
        m.entry(x)
            .and_modify(|v: &mut Vec<usize>| v.push(i))
            .or_insert(vec![i]);
    }

    let mut l = m.iter().collect::<Vec<_>>();
    l.sort_by(|a, b| b.0.cmp(a.0));

    let mut result = vec![0; N];
    let mut sum = 0;

    for (x, v) in l {
        for i in v {
            result[*i] = sum;
        }

        sum += *x * v.len();
    }

    result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5
1 4 1 4 2
"#
            ),
            r#"10 0 10 0 8"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
31 42 59 26 53 58 97 93 23 54
"#
            ),
            r#"456 414 190 487 361 249 0 97 513 307"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"50
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
"#
            ),
            r#"0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"#
        );
    }
}
