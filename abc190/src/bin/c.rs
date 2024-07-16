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
        N: usize,
        M: usize,
        AB: [(usize, usize); M],
        K: usize,
        CD: [(usize, usize); K],
    }

    let mut result = 0;

    for i in 0..2_usize.pow(K as u32) {
        let mut dishes = vec![0; N];

        for j in 0..K {
            if (i >> j) & 1 != 0 {
                dishes[CD[j].1 - 1] += 1;
            } else {
                dishes[CD[j].0 - 1] += 1;
            }
        }

        let mut sum = 0;

        for j in 0..M {
            if dishes[AB[j].0 - 1] > 0 && dishes[AB[j].1 - 1] > 0 {
                sum += 1;
            }
        }

        result = cmp::max(result, sum);
    }

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 4
1 2
1 3
2 4
3 4
3
1 2
1 3
2 3
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 4
1 2
1 3
2 4
3 4
4
3 4
1 2
2 4
2 4
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6 12
2 3
4 6
1 2
4 5
2 6
1 5
4 5
1 3
1 2
2 6
2 3
2 5
5
3 5
1 4
2 6
4 6
5 6
"#
            ),
            r#"9"#
        );
    }
}
