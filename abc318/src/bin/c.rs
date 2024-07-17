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
        D: usize,
        P: usize,
        F: [usize; N],
    }

    let mut f = F;
    f.sort_by(|a, b| b.cmp(a));

    let mut result = 0;

    for i in (0..N).step_by(D) {
        let sum = f.iter().skip(i).take(D).sum();
        result += cmp::min(sum, P)
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
                r#"5 2 10
7 1 6 3 6
"#
            ),
            r#"20"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 1 10
1 2 3
"#
            ),
            r#"6"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8 3 1000000000
1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000
"#
            ),
            r#"3000000000"#
        );
    }
}
