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
        _N: usize,
        S: Chars,
        Q: usize,
        cd: [(char, char); Q],
    }

    let mut v = ('a'..='z').collect::<Vec<_>>();

    for (c, d) in cd {
        for x in v.iter_mut() {
            if *x == c {
                *x = d;
            }
        }
    }

    let mut result = S;

    for c in result.iter_mut() {
        *c = v[(u32::from(*c) - 97) as usize];
    }

    result.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7
atcoder
4
r a
t e
d v
a r
"#
            ),
            r#"recover"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
abc
4
a a
s k
n n
z b
"#
            ),
            r#"abc"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"34
supercalifragilisticexpialidocious
20
g c
l g
g m
c m
r o
s e
a a
o f
f s
e t
t l
d v
p k
v h
x i
h n
n j
i r
s i
u a
"#
            ),
            r#"laklimamriiamrmrllrmlrkramrjimrial"#
        );
    }
}
