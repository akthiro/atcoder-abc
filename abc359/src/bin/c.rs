#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
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
        SX: isize,
        SY: isize,
        TX: isize,
        TY: isize,
    }

    let mut sx = SX;
    let sy = SY;

    let mut tx = TX;
    let ty = TY;

    if (sx + sy) % 2 == 1 {
        sx -= 1;
    }

    if (tx + ty) % 2 == 1 {
        tx -= 1;
    }

    let dx = (sx - tx).abs();
    let dy = (sy - ty).abs();

    let result = (dy + cmp::max(dx, dy)) / 2;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 0
2 5
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 1
4 1
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2552608206527595 5411232866732612
771856005518028 7206210729152763
"#
            ),
            r#"1794977862420151"#
        );
    }
}
