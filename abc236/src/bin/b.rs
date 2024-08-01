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
        A: [usize; 4 * N - 1],
    }

    let mut v = vec![0_usize; N];

    for n in A {
        v[n - 1] += 1;
    }

    for (i, n) in v.iter().enumerate() {
        if *n == 3 {
            return (i + 1).to_string();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
1 3 2 3 3 2 2 1 1 1 2
"#
            ),
            r#"3"#
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
3 2 1 1 2 4 4 4 4 3 1 3 2 1 3
"#
            ),
            r#"2"#
        );
    }
}
