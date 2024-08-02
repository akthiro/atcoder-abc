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
        A: [usize; 3 * N],
    }

    let mut v = vec![vec![]; N];

    for (i, n) in A.iter().enumerate() {
        v[n - 1].push(i);
    }

    let mut m = HashMap::new();

    for (i, v) in v.iter().enumerate() {
        let mut v = v.clone();
        v.sort();
        m.insert(i, v[1]);
    }

    let mut result = m.iter().collect::<Vec<_>>();
    result.sort_by(|a, b| a.1.cmp(b.1));

    result
        .iter()
        .map(|(i, _)| (*i + 1).to_string())
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
                r#"3
1 1 3 2 3 2 2 3 1
"#
            ),
            r#"1 3 2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1
1 1 1
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4
2 3 4 3 4 1 3 1 1 4 2 2
"#
            ),
            r#"3 4 1 2"#
        );
    }
}
