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
        N: usize,
        A: [usize; N],
    }

    let set: HashSet<&usize> = HashSet::from_iter(A.iter());

    if A.len() == set.len() {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5
2 6 1 4 5
"#
            ),
            r#"YES"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"6
4 1 3 1 6 2
"#
            ),
            r#"NO"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2
10000000 10000000
"#
            ),
            r#"NO"#
        );
    }
}
