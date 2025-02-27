///comes from:
/// https://www.codewars.com/kata/54d496788776e49e6b00052f/train/rust

/// generates the tuple list
fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut sums = vec![0];

    for &num in &l {
        let new_sums: Vec<_> = sums.iter().map(|&s| s + num).collect();
        sums.extend(new_sums);
    }
    dbg!(sums.clone());
    sums.remove(0);
    sums.dedup();

    let result: Vec<(i64, i64)> = sums
        .into_iter()
        .map(|sum| (smallest_prime_factor(sum), sum))
        .collect();

    result
}

pub fn smallest_prime_factor(n: i64) -> i64 {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return i;
        }
        i += 1;
    }
    n // If no factor is found, n itself is prime
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    // testing(
    //     vec![15, 21, 24, 30, 45],
    //     vec![(2, 54), (3, 135), (5, 90), (7, 21)],
    // );
}
