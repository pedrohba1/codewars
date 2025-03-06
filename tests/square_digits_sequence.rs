use std::collections::HashMap;

fn square_digits_sequence(a0: u32) -> usize {
    let mut map: HashMap<u32, bool> = HashMap::new();
    let mut an = a0;

    let mut i = 0;
    while !(map.get(&an).unwrap_or(&false)) {
        i += 1;
        map.insert(an, true);
        let digits = an
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>()
            .into_iter();

        an = 0;
        for d in digits {
            an += d.pow(2);
        }
    }
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(square_digits_sequence(16), 9);
        assert_eq!(square_digits_sequence(103), 4);
        assert_eq!(square_digits_sequence(1), 2);
    }
}
