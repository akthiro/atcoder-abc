#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
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
        M: usize,
        A: [usize; N],
    }

    let total = A.iter().sum::<usize>();

    if total <= M {
        return "infinite".to_string();
    }

    let mut result = 0;
    let mut max = total;

    while result + 1 < max {
        let mid = (result + max) / 2;

        if A.iter().map(|n| cmp::min(*n, mid)).sum::<usize>() <= M {
            result = mid;
        } else {
            max = mid;
        }
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
                r#"4 8
1 3 2 4
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 20
5 3 2
"#
            ),
            r#"infinite"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 23
2 5 6 5 2 1 7 9 7 2
"#
            ),
            r#"2"#
        );
    }
}
