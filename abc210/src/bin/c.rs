#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
    collections::HashMap,
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
        c: [usize; N],
    }

    let mut result = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();

    for n in c.iter().take(K) {
        map.entry(*n).and_modify(|n| *n += 1).or_insert(1);
    }

    result = cmp::max(result, map.len());

    for i in K..N {
        match map.get_mut(&c[i - K]) {
            Some(1) => {
                map.remove(&c[i - K]);
            }
            Some(n) => {
                *n -= 1;
            }
            _ => unreachable!(),
        };

        match map.get_mut(&c[i]) {
            Some(n) => {
                *n += 1;
            }
            None => {
                map.insert(c[i], 1);
            }
        }

        result = cmp::max(result, map.len());
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
                r#"7 3
1 2 1 2 3 3 1
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 5
4 4 4 4 4
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 6
304621362 506696497 304621362 506696497 834022578 304621362 414720753 304621362 304621362 414720753
"#
            ),
            r#"4"#
        );
    }
}
