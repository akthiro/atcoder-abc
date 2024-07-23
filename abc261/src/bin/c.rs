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
        S: [String; N],
    }

    let mut result = vec![];
    let mut map = HashMap::new();

    for s in S {
        match map.get_mut(&s) {
            None => {
                map.insert(s.clone(), 1);
                result.push(s);
            }
            Some(cnt) => {
                result.push(format!("{}({})", s, cnt));
                *cnt += 1;
            }
        }
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
                r#"5
newfile
newfile
newfolder
newfile
newfolder
"#
            ),
            r#"newfile
newfile(1)
newfolder
newfile(2)
newfolder(1)"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"11
a
a
a
a
a
a
a
a
a
a
a
"#
            ),
            r#"a
a(1)
a(2)
a(3)
a(4)
a(5)
a(6)
a(7)
a(8)
a(9)
a(10)"#
        );
    }
}
