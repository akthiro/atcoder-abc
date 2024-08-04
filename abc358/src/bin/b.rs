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
        A: usize,
        T: [usize; N],
    }

    let mut result = vec![];
    let mut current = 0;

    for n in T {
        current = cmp::max(current, n) + A;
        result.push(current);
    }

    result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 4
0 2 10
"#
            ),
            r#"4
8
14"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 3
1 4 7
"#
            ),
            r#"4
7
10"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 50000
120190 165111 196897 456895 540000 552614 561627 743796 757613 991216
"#
            ),
            r#"170190
220190
270190
506895
590000
640000
690000
793796
843796
1041216"#
        );
    }
}
