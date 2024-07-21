#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::VecDeque,
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
        n: usize,
        a: [usize; n],
    }

    let mut deq = VecDeque::new();

    for (i, n) in a.iter().enumerate() {
        if i % 2 == 0 {
            deq.push_back(n);
        } else {
            deq.push_front(n);
        }
    }

    let mut result = deq.iter().collect::<Vec<_>>();

    if n % 2 != 0 {
        result.reverse();
    }

    result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
1 2 3 4
"#
            ),
            r#"4 2 1 3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
1 2 3
"#
            ),
            r#"3 1 2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1
1000000000
"#
            ),
            r#"1000000000"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"6
0 6 7 6 7 0
"#
            ),
            r#"0 6 6 0 7 7"#
        );
    }
}
