#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    let mut a = vec![];
    let mut r = vec![];

    for _ in 0..M {
        input! {
            C: usize,
            A: [usize; C],
            R: char,
        }

        a.push(A.iter().cloned().map(|n| n - 1).collect::<Vec<_>>());
        r.push(R);
    }

    let mut result = 0;

    for bit in 0..1 << N {
        let mut ok = true;

        for i in 0..M {
            let mut cnt = 0;

            for j in &a[i] {
                if bit & (1 << j) != 0 {
                    cnt += 1;
                }
            }

            if (cnt < K && r[i] == 'o') || (cnt >= K && r[i] == 'x') {
                ok = false;
            }
        }

        if ok {
            result += 1;
        }
    }

    println!("{}", result);
}
