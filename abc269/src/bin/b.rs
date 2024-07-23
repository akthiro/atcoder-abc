#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        S: [Chars; 10],
    }

    let mut l = vec![];

    for (i, cs) in S.iter().enumerate().take(10) {
        for (j, c) in cs.iter().enumerate().take(10) {
            if *c == '#' {
                l.push((i, j));
            }
        }
    }

    l.sort_by(|a, b| a.0.cmp(&b.0));
    let a = l[0].0 + 1;
    let b = l[l.len() - 1].0 + 1;
    l.sort_by(|a, b| a.1.cmp(&b.1));
    let c = l[0].1 + 1;
    let d = l[l.len() - 1].1 + 1;

    format!("{} {}\n{} {}", a, b, c, d)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"..........
..........
..........
..........
...######.
...######.
...######.
...######.
..........
..........
"#
            ),
            r#"5 8
4 9"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"..........
..#.......
..........
..........
..........
..........
..........
..........
..........
..........
"#
            ),
            r#"2 2
3 3"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"##########
##########
##########
##########
##########
##########
##########
##########
##########
##########
"#
            ),
            r#"1 10
1 10"#
        );
    }
}
