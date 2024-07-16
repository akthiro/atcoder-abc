#![allow(non_snake_case)]

use itertools::Itertools;
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
        P: [usize; N],
        Q: [usize; N],
    }

    let mut l = (1..=N).permutations(N).collect::<Vec<_>>();
    l.sort();

    let mut a = 0;
    let mut b = 0;

    for (i, l) in l.iter().enumerate() {
        if P == *l {
            a = i;
        }

        if Q == *l {
            b = i;
        }
    }

    format!("{}", (a as isize - b as isize).abs())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
1 3 2
3 1 2
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8
7 3 5 4 2 1 6 8
3 8 2 5 4 6 7 1
"#
            ),
            r#"17517"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3
1 2 3
1 2 3
"#
            ),
            r#"0"#
        );
    }
}
