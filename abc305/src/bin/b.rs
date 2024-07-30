#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
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
        p: char,
        q: char,
    }

    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('B', 1);
    map.insert('C', 2);
    map.insert('D', 3);
    map.insert('E', 4);
    map.insert('F', 5);
    map.insert('G', 6);

    let start = cmp::min(map[&p], map[&q]);
    let end = cmp::max(map[&p], map[&q]);

    [3, 1, 4, 1, 5, 9][start..end]
        .iter()
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"A C
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"G B
"#
            ),
            r#"20"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"C F
"#
            ),
            r#"10"#
        );
    }
}
