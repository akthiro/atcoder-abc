#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
use std::{
    collections::HashSet,
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
        H: usize,
        W: usize,
        G: [Chars; H],
    }

    let mut s: HashSet<(usize, usize)> = HashSet::new();

    let mut i = 0;
    let mut j = 0;

    loop {
        if s.contains(&(i, j)) {
            return "-1".to_string();
        }

        s.insert((i, j));

        match G[i][j] {
            'U' if i != 0 => {
                i -= 1;
            }
            'D' if i != H - 1 => {
                i += 1;
            }
            'L' if j != 0 => {
                j -= 1;
            }
            'R' if j != W - 1 => {
                j += 1;
            }
            _ => return format!("{} {}", i + 1, j + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 3
RDU
LRU
"#
            ),
            r#"1 3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 3
RRD
ULL
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"9 44
RRDDDDRRRDDDRRRRRRDDDRDDDDRDDRDDDDDDRRDRRRRR
RRRDLRDRDLLLLRDRRLLLDDRDLLLRDDDLLLDRRLLLLLDD
DRDLRLDRDLRDRLDRLRDDLDDLRDRLDRLDDRLRRLRRRDRR
DDLRRDLDDLDDRLDDLDRDDRDDDDRLRRLRDDRRRLDRDRDD
RDLRRDLRDLLLLRRDLRDRRDRRRDLRDDLLLLDDDLLLLRDR
RDLLLLLRDLRDRLDDLDDRDRRDRLDRRRLDDDLDDDRDDLDR
RDLRRDLDDLRDRLRDLDDDLDDRLDRDRDLDRDLDDLRRDLRR
RDLDRRLDRLLLLDRDRLLLRDDLLLLLRDRLLLRRRRLLLDDR
RRRRDRDDRRRDDRDDDRRRDRDRDRDRRRRRRDDDRDDDDRRR
"#
            ),
            r#"9 5"#
        );
    }
}
