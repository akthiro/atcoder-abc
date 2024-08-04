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
        T: usize,
        A: [usize; T],
    }

    let mut bingo = vec![vec![usize::MAX; N]; N];

    for i in 0..T {
        bingo[(A[i] - 1) / N][(A[i] - 1) % N] = i + 1;
    }

    let mut result = usize::MAX;

    #[allow(clippy::needless_range_loop)]
    for x in 0..N {
        let mut max = 0;

        for y in 0..N {
            max = max.max(bingo[x][y]);
        }

        result = result.min(max);
    }

    #[allow(clippy::needless_range_loop)]
    for y in 0..N {
        let mut max = 0;

        for x in 0..N {
            max = max.max(bingo[x][y]);
        }

        result = result.min(max);
    }

    let mut x = 0;
    let mut y = 0;
    let mut max = 0;

    #[allow(clippy::explicit_counter_loop)]
    for _ in 0..N {
        max = max.max(bingo[x][y]);
        x += 1;
        y += 1;
    }

    result = result.min(max);

    let mut x = 0;
    let mut y = (N - 1) as isize;
    let mut max = 0;

    #[allow(clippy::explicit_counter_loop)]
    for _ in 0..N {
        max = max.max(bingo[x][y as usize]);
        x += 1;
        y -= 1;
    }

    result = result.min(max);

    if result == usize::MAX {
        (-1).to_string()
    } else {
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 5
5 1 8 9 7
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 5
4 2 9 7 5
"#
            ),
            r#"-1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4 12
13 9 6 5 2 7 16 14 8 3 10 11
"#
            ),
            r#"9"#
        );
    }
}
