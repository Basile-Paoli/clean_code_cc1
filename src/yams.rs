use std::collections::HashMap;

type Dice = [u8; 5];

pub fn calculate_yams_score(dice: &Dice) -> u32 {
    if contains_four_of_a_kind(dice) {
        35
    } else if contains_full_house(dice) {
        30
    } else if contains_three_of_a_kind(dice) {
        28
    } else {
        let sum: u8 = dice.iter().sum();
        sum as u32
    }
}

fn contains_four_of_a_kind(dice: &Dice) -> bool {
    for die in dice {
        if dice.iter().filter(|x| *x == die).count() >= 4 {
            return true;
        }
    }
    false
}

fn contains_full_house(dice: &Dice) -> bool {
    let mut counts = HashMap::new();
    for die in dice {
        *counts.entry(die).or_insert(0) += 1;
    }
    counts.values().any(|&count| count == 3) && counts.values().any(|&count| count == 2)
}

fn contains_three_of_a_kind(dice: &Dice) -> bool {
    for die in dice {
        if dice.iter().filter(|x| *x == die).count() >= 3 {
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
    fn test_four_of_a_kind() {
        let dice = [4, 4, 4, 4, 1];
        assert_eq!(calculate_yams_score(&dice), 35);
    }

    #[test]
    fn test_full_house() {
        let dice = [2, 2, 3, 3, 3];
        assert_eq!(calculate_yams_score(&dice), 30);
    }

    #[test]
    fn test_chance() {
        let dice = [1, 2, 3, 4, 5];
        assert_eq!(calculate_yams_score(&dice), 15);
    }
}
