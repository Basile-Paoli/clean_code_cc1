type Dice = [u8; 5];

pub fn calculate_yams_score(dice: &Dice) -> u32 {
    if contains_three_of_a_kind(dice) {
        28
    } else {
        let sum: u8 = dice.iter().sum();
        sum as u32
    }
}

fn contains_three_of_a_kind(dice: &Dice) -> bool {
    for value in dice {
        if dice.iter().filter(|x| *x == value).count() >= 3 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::calculate_yams_score;

    #[test]
    fn test_three_of_a_kind() {
        let dice = [3, 3, 3, 2, 5];
        assert_eq!(calculate_yams_score(&dice), 28);
    }

    #[test]
    fn test_chance() {
        let dice = [1, 2, 3, 4, 5];
        assert_eq!(calculate_yams_score(&dice), 15);
    }
}
