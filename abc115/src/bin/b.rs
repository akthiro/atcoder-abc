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
        p: [usize; N],
    }

    let mut p = p;
    p.sort();

    let max = p[N - 1];
    let result = p.iter().take(N - 1).sum::<usize>() + max / 2;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
4980
7980
6980
"#
            ),
            r#"15950"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
4320
4320
4320
4320
"#
            ),
            r#"15120"#
        );
    }
}
