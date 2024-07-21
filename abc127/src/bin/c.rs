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
        N: isize,
        M: usize,
        LR: [(isize, isize); M],
    }

    let mut min = 1;
    let mut max = N;

    for (l, r) in LR.iter() {
        min = cmp::max(min, *l);
        max = cmp::min(max, *r);
    }

    cmp::max(max - min + 1, 0).to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 2
1 3
2 4
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10 3
3 6
5 7
6 9
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"100000 1
1 100000
"#
            ),
            r#"100000"#
        );
    }
}
