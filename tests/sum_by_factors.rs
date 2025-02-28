///comes from:
/// https://www.codewars.com/kata/54d496788776e49e6b00052f/train/rust
/// generates the tuple list
use std::collections::{HashMap, HashSet};

/// Function to get prime factors of a number
fn prime_factors(mut n: i64) -> HashSet<i64> {
    let mut factors = HashSet::new();
    n = n.abs(); // Work with absolute value

    while n % 2 == 0 {
        factors.insert(2);
        n /= 2;
    }

    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.insert(i);
            n /= i;
        }
        i += 2;
    }

    if n > 1 {
        factors.insert(n);
    }

    factors
}

/// Find the most unique number (least occurrences across prime factor lists)
fn find_most_unique(factor_map: &HashMap<i64, Vec<i64>>) -> Option<i64> {
    let mut occurrence_count: HashMap<i64, usize> = HashMap::new();

    // Count occurrences of each number across all factor lists
    for numbers in factor_map.values() {
        for &num in numbers {
            *occurrence_count.entry(num).or_insert(0) += 1;
        }
    }

    // Find the number with the least occurrences, using the smallest number in case of ties
    occurrence_count
        .iter()
        .min_by_key(|&(num, count)| (*count, *num))
        .map(|(num, _)| *num)
}

/// Function to assign numbers to primes while prioritizing uniqueness
fn assign_numbers_to_primes(mut factor_map: HashMap<i64, Vec<i64>>) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    let mut assigned = HashSet::new();

    while !factor_map.is_empty() {
        if let Some(most_unique) = find_most_unique(&factor_map) {
            // Find the smallest prime factor this number belongs to
            let smallest_prime = factor_map
                .iter()
                .filter(|(_, nums)| nums.contains(&most_unique))
                .map(|(prime, _)| *prime)
                .min()
                .unwrap(); // This is safe since we just found a unique number

            // Assign it to this prime
            result.push((smallest_prime, most_unique));
            assigned.insert(most_unique);

            // Remove the number from all factor lists
            for numbers in factor_map.values_mut() {
                numbers.retain(|&x| x != most_unique);
            }

            // Remove empty factor entries
            factor_map.retain(|_, nums| !nums.is_empty());
        }
    }

    dbg!(result.clone());
    result
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut sums = vec![0];

    for &num in &l {
        let new_sums: Vec<_> = sums.iter().map(|&s| s + num).collect();
        sums.extend(new_sums);
    }
    dbg!(sums.clone());
    sums.remove(0);

    //TODO: Havee a way to generate the values without
    // having to dedup them
    sums.dedup();

    let mut factor_map: HashMap<i64, Vec<i64>> = HashMap::new();

    for &num in &sums {
        for factor in prime_factors(num) {
            factor_map.entry(factor).or_insert(Vec::new()).push(num);
        }
    }

    dbg!(factor_map.clone());
    for &num in &l {
        for factor in prime_factors(num) {
            factor_map.entry(factor).or_insert(Vec::new()).push(num);
        }
    }

    assign_numbers_to_primes(factor_map)
}

#[cfg(test)]
mod tests {
    use super::sum_of_divided;

    /// Helper function for testing
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
    //
    // #[test]
    // fn test_with_negative_numbers() {
    //     run_test(vec![-15, -30, 45], vec![(2, -30), (3, 0), (5, 0)]);
    // }
    //
    // #[test]
    // fn test_with_large_numbers() {
    //     run_test(vec![100, 200, 300], vec![(2, 600), (5, 600)]);
    // }
    //
    // #[test]
    // fn test_with_primes() {
    //     run_test(
    //         vec![7, 11, 13, 17],
    //         vec![(7, 7), (11, 11), (13, 13), (17, 17)],
    //     );
    // }
    //
    // #[test]
    // fn test_with_zeros() {
    //     run_test(vec![0, 5, 10], vec![(2, 10), (5, 15)]);
    // }
}
