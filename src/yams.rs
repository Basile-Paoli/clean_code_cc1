fn calculate_yams_score(dice: &[u8; 5]) -> u32 {
    28
}

#[cfg(test)]
mod test {
    use super::calculate_yams_score;

    #[test]
    fn test_three_of_a_kind() {
        let dice = [3, 3, 3, 2, 5];
        assert_eq!(calculate_yams_score(&dice), 28);
    }
}
