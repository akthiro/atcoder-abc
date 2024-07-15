#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashMap,
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

    let mut cnt = HashMap::new();
    let mut l = vec![];

    for n in A {
        cnt.entry(n).and_modify(|n| *n += 1).or_insert(1);

        if cnt[&n] == 2 {
            cnt.remove(&n);
            l.push(n);
        }
    }

    l.sort_by(|a, b| b.cmp(a));

    let result = if l.len() >= 2 { l[0] * l[1] } else { 0 };

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"6
3 1 2 4 2 1
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
1 2 3 4
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10
3 3 3 3 4 4 4 5 5 5
"#
            ),
            r#"20"#
        );
    }
}
