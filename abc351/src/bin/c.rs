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
        N: usize,
        A: [usize; N],
    }

    let mut queue = VecDeque::new();

    for n in A {
        queue.push_back(n);

        while queue.len() > 1 {
            let last1 = queue.pop_back().unwrap();
            let last2 = queue.pop_back().unwrap();

            if last1 != last2 {
                queue.push_back(last2);
                queue.push_back(last1);
                break;
            } else {
                queue.push_back(last1 + 1);
            }
        }
    }

    format!("{}", queue.len())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7
2 1 1 3 5 3 3
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
0 0 0 1 2
"#
            ),
            r#"4"#
        );
    }
}
