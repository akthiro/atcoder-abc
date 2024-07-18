#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::VecDeque,
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
        N: u32,
        A: [usize; 2_usize.pow(N)],
    }

    let mut a = VecDeque::from_iter(A.iter().enumerate());

    loop {
        let x = a.pop_front().unwrap();
        let y = a.pop_front().unwrap();

        let (win, lose) = if x.1 < y.1 { (y, x) } else { (x, y) };

        a.push_back(win);

        if a.len() == 1 {
            return (lose.0 + 1).to_string();
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
                r#"2
1 4 2 5
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
3 1 5 4
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4
6 13 12 5 3 7 10 11 16 9 8 15 2 1 14 4
"#
            ),
            r#"2"#
        );
    }
}
