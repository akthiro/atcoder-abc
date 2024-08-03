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
        A: [usize; N],
    }

    let mut a = A.clone();
    a.sort_by(|a, b| b.cmp(a));

    let max1 = a[0];
    let max2 = a[1];

    let mut result = vec![];

    for n in A {
        if n == max1 {
            result.push(max2);
        } else {
            result.push(max1);
        }
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
                r#"3
1
4
3
"#
            ),
            r#"4
3
4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
5
5
"#
            ),
            r#"5
5"#
        );
    }
}
