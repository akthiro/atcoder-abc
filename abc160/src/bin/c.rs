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
        K: usize,
        N: usize,
        A: [usize; N],
    }

    let mut a = vec![];

    for i in 0..N - 1 {
        a.push(A[i + 1] - A[i]);
    }

    a.push(K - A.last().unwrap() + A.first().unwrap());

    let sum = a.iter().sum::<usize>();
    let max = a.iter().max().unwrap();

    let result = sum - max;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"20 3
5 10 15
"#
            ),
            r#"10"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"20 3
0 5 15
"#
            ),
            r#"10"#
        );
    }
}
