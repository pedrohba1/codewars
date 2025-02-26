fn disemvowel(s: &str) -> String {
    s.replace(&['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'][..], "")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }
}
