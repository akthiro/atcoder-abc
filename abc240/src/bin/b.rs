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
        a: [usize; N],
    }

    let set: HashSet<&usize> = HashSet::from_iter(a.iter());

    format!("{}", set.len())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"6
1 4 1 2 2 1
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
1
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"11
3 1 4 1 5 9 2 6 5 3 5
"#
            ),
            r#"7"#
        );
    }
}
