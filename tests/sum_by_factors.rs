///comes from:
/// https://www.codewars.com/kata/54d496788776e49e6b00052f/train/rust
use std::collections::HashSet;

fn prime_factors(mut n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    n = n.abs();

    while n % 2 == 0 {
        if factors.is_empty() || factors.last() != Some(&2) {
            factors.push(2);
        }
        n /= 2;
    }

    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            if factors.is_empty() || factors.last() != Some(&i) {
                factors.push(i);
            }
            n /= i;
        }
        i += 2;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

fn combine(arr1: Vec<i64>, arr2: &mut Vec<i64>) {
    let mut set: HashSet<i64> = arr2.iter().cloned().collect();
    for a in arr1 {
        if !set.contains(&a) {
            arr2.push(a);
            set.insert(a);
        }
    }
}

fn sum_of_divided(lst: Vec<i64>) -> Vec<(i64, i64)> {
    let mut factors: Vec<i64> = Vec::new();
    let mut sums: Vec<(i64, i64)> = Vec::new();

    for &num in &lst {
        combine(prime_factors(num), &mut factors);
    }

    for &factor in &factors {
        let mut msum = 0;
        for &num in &lst {
            if num % factor == 0 {
                msum += num;
            }
        }
        sums.push((factor, msum));
    }

    sums.sort_by_key(|&(factor, _)| factor);

    sums
}

#[cfg(test)]
mod tests {
    use super::sum_of_divided;

    fn run_test(input: Vec<i64>, expected: Vec<(i64, i64)>) {
        assert_eq!(sum_of_divided(input), expected);
    }

    #[test]
    fn test_basic_case_1() {
        run_test(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    }

    #[test]
    fn test_basic_case_2() {
        run_test(
            vec![15, 21, 24, 30, 45],
            vec![(2, 54), (3, 135), (5, 90), (7, 21)],
        );
    }
    #[test]
    fn test_with_negative_numbers() {
        run_test(vec![-15, -30, 45], vec![(2, -30), (3, 0), (5, 0)]);
    }

    #[test]
    fn test_with_primes() {
        run_test(
            vec![7, 11, 13, 17],
            vec![(7, 7), (11, 11), (13, 13), (17, 17)],
        );
    }
}
