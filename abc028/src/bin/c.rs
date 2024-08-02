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
        A: usize,
        B: usize,
        C: usize,
        D: usize,
        E: usize,
    }

    let v = [A, B, C, D, E];

    let mut sums = HashSet::new();

    for i in 0..5 {
        for j in i + 1..5 {
            for k in j + 1..5 {
                sums.insert(v[i] + v[j] + v[k]);
            }
        }
    }

    let mut result = sums.iter().collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));

    format!("{}", result[2])
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"1 2 3 4 5
"#
            ),
            r#"10"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 2 3 5 8
"#
            ),
            r#"14"#
        );
    }
}
