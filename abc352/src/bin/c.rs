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
        AB: [(usize, usize); N],
    }

    let total = AB.iter().map(|(a, _)| a).sum::<usize>();
    let head = AB.iter().map(|(a, b)| b - a).max().unwrap();

    let result = total + head;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
4 10
5 8
2 9
"#
            ),
            r#"18"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
1 1
1 1
1 1
1 1
1 1
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10
690830957 868532399
741145463 930111470
612846445 948344128
540375785 925723427
723092548 925021315
928915367 973970164
563314352 832796216
562681294 868338948
923012648 954764623
691107436 891127278
"#
            ),
            r#"7362669937"#
        );
    }
}
