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
        ABCD: [(usize, usize, usize, usize); N],
    }

    let mut table = vec![vec![false; 100]; 100];

    for (a, b, c, d) in ABCD {
        for row in table.iter_mut().take(b).skip(a) {
            for cell in row.iter_mut().take(d).skip(c) {
                *cell = true;
            }
        }
    }

    let result = table
        .iter()
        .map(|row| row.iter().filter(|cell| **cell).count())
        .sum::<usize>();

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
0 5 1 3
1 4 0 5
2 5 2 4
"#
            ),
            r#"20"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
0 100 0 100
0 100 0 100
"#
            ),
            r#"10000"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3
0 1 0 1
0 3 0 5
5 10 0 10
"#
            ),
            r#"65"#
        );
    }
}
