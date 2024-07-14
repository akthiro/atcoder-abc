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
        K: usize,
        A: [usize; N],
    }

    let mut l = vec![0; N];

    for n in A {
        l[n - 1] += 1;
    }

    l.sort();

    let l = l.iter().cloned().filter(|n| *n > 0).collect::<Vec<_>>();

    if l.len() <= K {
        "0".to_string()
    } else {
        l.iter()
            .cloned()
            .take(l.len() - K)
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 2
1 1 2 2 5
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 4
1 1 2 2
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 3
5 1 3 2 4 1 1 2 3 4
"#
            ),
            r#"3"#
        );
    }
}
