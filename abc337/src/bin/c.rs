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
        A: [isize; N],
    }

    let mut v = vec![N; N];

    let mut front = 0;

    for (i, x) in A.iter().enumerate() {
        if *x == -1 {
            front = i;
        } else {
            v[*x as usize - 1] = i;
        }
    }

    let mut result = vec![];

    while front < N {
        result.push(front + 1);
        front = v[front];
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
                r#"6
4 1 -1 5 3 2
"#
            ),
            r#"3 5 4 1 2 6"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
-1 1 2 3 4 5 6 7 8 9
"#
            ),
            r#"1 2 3 4 5 6 7 8 9 10"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"30
3 25 20 6 18 12 26 1 29 -1 21 17 23 9 8 30 10 15 22 27 4 13 5 11 16 24 28 2 19 7
"#
            ),
            r#"10 17 12 6 4 21 11 24 26 7 30 16 25 2 28 27 20 3 1 8 15 18 5 23 13 22 19 29 9 14"#
        );
    }
}
