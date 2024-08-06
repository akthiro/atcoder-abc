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
        A: [[usize; N]; N],
    }

    let mut result = vec![];

    for v in A {
        let mut item = vec![];

        for (i, x) in v.iter().enumerate() {
            if *x == 1 {
                item.push(i + 1);
            }
        }

        result.push(
            item.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        );
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
0 1 1 0
1 0 0 1
1 0 0 0
0 1 0 0
"#
            ),
            r#"2 3
1 4
1
2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
0 0
0 0
"#
            ),
            r#"
"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5
0 1 0 1 1
1 0 0 1 0
0 0 0 0 1
1 1 0 0 1
1 0 1 1 0
"#
            ),
            r#"2 4 5
1 4
5
1 2 5
1 3 4"#
        );
    }
}
