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
        a: isize,
        b: isize,
        c: isize,
        d: isize,
        e: isize,
        f: isize,
        g: isize,
        h: isize,
        i: isize,
        j: isize,
        k: isize,
        l: isize,
    }

    let x = cmp::max(cmp::min(d, j) - cmp::max(a, g), 0);
    let y = cmp::max(cmp::min(e, k) - cmp::max(b, h), 0);
    let z = cmp::max(cmp::min(f, l) - cmp::max(c, i), 0);

    if x * y * z > 0 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"0 0 0 4 5 6
2 3 4 5 6 7
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"0 0 0 2 2 2
0 0 2 2 2 4
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"0 0 0 1000 1000 1000
10 10 10 100 100 100
"#
            ),
            r#"Yes"#
        );
    }
}
