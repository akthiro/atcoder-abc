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
        N: usize,
        A: [Chars; N],
        B: [Chars; N],
    }

    for i in 0..N {
        for j in 0..N {
            if A[i][j] != B[i][j] {
                return format!("{} {}", i + 1, j + 1);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
abc
def
ghi
abc
bef
ghi
"#
            ),
            r#"2 1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1
f
q
"#
            ),
            r#"1 1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10
eixfumagit
vtophbepfe
pxbfgsqcug
ugpugtsxzq
bvfhxyehfk
uqyfwtmglr
jaitenfqiq
acwvufpfvv
jhaddglpva
aacxsyqvoj
eixfumagit
vtophbepfe
pxbfgsqcug
ugpugtsxzq
bvfhxyehok
uqyfwtmglr
jaitenfqiq
acwvufpfvv
jhaddglpva
aacxsyqvoj
"#
            ),
            r#"5 9"#
        );
    }
}
