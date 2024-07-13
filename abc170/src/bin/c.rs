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
        X: isize,
        N: usize,
        p: [isize; N],
    }

    let set: HashSet<isize> = HashSet::from_iter(p.iter().cloned());

    let mut result = X;
    let mut diff = isize::MAX;

    for n in 0..=101 {
        if set.contains(&n) {
            continue;
        }

        let m = (n - X).abs();

        if diff > m {
            result = n;
            diff = m;
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
                r#"6 5
4 7 10 6 5
"#
            ),
            r#"8"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10 5
4 7 10 6 5
"#
            ),
            r#"9"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100 0

"#
            ),
            r#"100"#
        );
    }
}
