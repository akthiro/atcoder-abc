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
        K: usize,
        ab: [(usize, usize); N],
    }

    let mut ab = ab;
    ab.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum = ab.iter().map(|(_, b)| *b).sum::<usize>();

    if sum <= K {
        return 1.to_string();
    }

    for i in 0..N {
        if sum <= K {
            return (ab[i - 1].0 + 1).to_string();
        }

        sum -= ab[i].1;
    }

    (ab.last().unwrap().0 + 1).to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4 8
6 3
2 5
1 9
4 2
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 100
6 3
2 5
1 9
4 2
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"15 158260522
877914575 2436426
24979445 61648772
623690081 33933447
476190629 62703497
211047202 71407775
628894325 31963982
822804784 50968417
430302156 82631932
161735902 80895728
923078537 7723857
189330739 10286918
802329211 4539679
303238506 17063340
492686568 73361868
125660016 50287940
"#
            ),
            r#"492686569"#
        );
    }
}
