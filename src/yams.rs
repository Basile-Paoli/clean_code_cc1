use std::collections::HashMap;

type Dice = [u8; 5];

enum CombinationResult {
    Matched(u32),
    NotMatched,
}

type CombinationChecker = fn(&Dice) -> CombinationResult;

fn calculate_yams_total_score(rounds: &[Dice]) -> u32 {
    rounds
        .iter()
        .map(|dice| calculate_yams_round_score(dice))
        .sum()
}

fn calculate_yams_round_score(dice: &Dice) -> u32 {
    let cases: Vec<CombinationChecker> = vec![
        check_four_of_a_kind,
        check_full_house,
        check_three_of_a_kind,
        check_straight,
    ];
    for case in cases {
        if let CombinationResult::Matched(score) = case(dice) {
            return score;
        }
    }

    calculate_chance_score(dice)
}

fn check_four_of_a_kind(dice: &Dice) -> CombinationResult {
    if contains_four_of_a_kind(dice) {
        CombinationResult::Matched(35)
    } else {
        CombinationResult::NotMatched
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

fn check_full_house(dice: &Dice) -> CombinationResult {
    if contains_full_house(dice) {
        CombinationResult::Matched(30)
    } else {
        CombinationResult::NotMatched
    }
}

fn contains_full_house(dice: &Dice) -> bool {
    let mut counts = HashMap::new();
    for die in dice {
        *counts.entry(die).or_insert(0) += 1;
    }
    counts.values().any(|&count| count == 3) && counts.values().any(|&count| count == 2)
}

fn check_three_of_a_kind(dice: &Dice) -> CombinationResult {
    if contains_three_of_a_kind(dice) {
        CombinationResult::Matched(28)
    } else {
        CombinationResult::NotMatched
    }
}

fn contains_three_of_a_kind(dice: &Dice) -> bool {
    for die in dice {
        if dice.iter().filter(|x| *x == die).count() >= 3 {
            return true;
        }
    }
    false
}

fn check_straight(dice: &Dice) -> CombinationResult {
    if contains_straight(dice) {
        CombinationResult::Matched(40)
    } else {
        CombinationResult::NotMatched
    }
}

fn contains_straight(dice: &Dice) -> bool {
    let mut sorted_dice = dice.clone();
    sorted_dice.sort();

    sorted_dice == [1, 2, 3, 4, 5] || sorted_dice == [2, 3, 4, 5, 6]
}

fn calculate_chance_score(dice: &Dice) -> u32 {
    let sum: u8 = dice.iter().sum();
    sum as u32
}

#[cfg(test)]
mod test {
    use super::calculate_yams_round_score;

    #[test]
    fn test_three_of_a_kind() {
        let dice = [3, 3, 3, 2, 5];
        assert_eq!(calculate_yams_round_score(&dice), 28);
    }

    #[test]
    fn test_four_of_a_kind() {
        let dice = [4, 4, 4, 4, 1];
        assert_eq!(calculate_yams_round_score(&dice), 35);
    }

    #[test]
    fn test_full_house() {
        let dice = [2, 2, 3, 3, 3];
        assert_eq!(calculate_yams_round_score(&dice), 30);
    }

    #[test]
    fn test_straight() {
        let dice = [1, 2, 3, 4, 5];
        assert_eq!(calculate_yams_round_score(&dice), 40);
    }

    #[test]
    fn test_chance() {
        let dice = [1, 2, 3, 4, 6];
        assert_eq!(calculate_yams_round_score(&dice), 16);
    }

    #[test]
    fn test_yams_total_score() {
        let rounds = vec![
            [3, 3, 3, 2, 5], // Three of a kind: 28
            [4, 4, 4, 4, 1], // Four of a kind: 35
            [2, 2, 3, 3, 3], // Full house: 30
            [1, 2, 3, 4, 5], // Straight: 40
            [1, 2, 3, 4, 6], // Chance: 16
        ];
        assert_eq!(super::calculate_yams_total_score(&rounds), 149);
    }
}
