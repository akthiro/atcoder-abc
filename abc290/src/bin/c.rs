#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
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
        N: usize,
        K: usize,
        A: [usize; N],
    }

    let a: HashSet<usize> = HashSet::from_iter(A);
    let mut a = a.into_iter().collect::<Vec<_>>();
    a.sort();

    let mut result = 0;

    for n in a {
        if n != result {
            break;
        }

        result += 1;
    }

    result = cmp::min(result, K);

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7 3
2 0 2 3 2 1 9
"#
            ),
            r#"3"#
        );
    }
}
