use std::collections::HashMap;

type Dice = [u8; 5];

#[derive(Clone, Copy)]
enum CombinationResult {
    Matched(u32),
    NotMatched,
}

type CombinationChecker = fn(&Dice) -> CombinationResult;
type RoundResult = (Combination, u32);
type CaseResult = (CombinationResult, Combination);

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Combination {
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    Straight,
    Yams,
    Chance,
}

const ORDERED_COMBINATIONS: [Combination; 6] = [
    Combination::Yams,
    Combination::Straight,
    Combination::FourOfAKind,
    Combination::FullHouse,
    Combination::ThreeOfAKind,
    Combination::Chance,
];

fn calculate_yams_total_score(rounds: &[Dice]) -> u32 {
    let mut available_combinations = ORDERED_COMBINATIONS.to_vec();

    let mut sum = 0;
    for dice in rounds {
        let result = calculate_yams_round_score(dice, &available_combinations);

        match result {
            Some((combination, score)) => {
                available_combinations.retain(|&c| c != combination);
                sum += score;
            }
            None => {}
        }
    }
    sum
}

fn calculate_yams_round_score(
    dice: &Dice,
    available_combinations: &[Combination],
) -> Option<(Combination, u32)> {
    let case_map: HashMap<Combination, CombinationChecker> = HashMap::from([
        (
            Combination::FourOfAKind,
            check_four_of_a_kind as CombinationChecker,
        ),
        (Combination::FullHouse, check_full_house),
        (Combination::ThreeOfAKind, check_three_of_a_kind),
        (Combination::Straight, check_straight),
        (Combination::Yams, check_yams),
        (Combination::Chance, check_chance),
    ]);

    let checks = available_combinations
        .iter()
        .filter_map(|combination| case_map.get_key_value(combination))
        .collect::<Vec<_>>();
    let case_results: Vec<CaseResult> = checks
        .iter()
        .map(|(combination, case)| (case(dice), **combination))
        .collect();
    let max_score = get_best_case(&case_results);

    max_score.and_then(|case| match case {
        (CombinationResult::Matched(score), combination) => Some((combination, score)),
        (CombinationResult::NotMatched, _) => None,
    })
}

fn get_best_case(cases: &[(CombinationResult, Combination)]) -> Option<CaseResult> {
    cases
        .iter()
        .max_by(|a, b| match a.0 {
            CombinationResult::NotMatched => std::cmp::Ordering::Less,
            CombinationResult::Matched(a_score) => match b.0 {
                CombinationResult::NotMatched => std::cmp::Ordering::Greater,
                CombinationResult::Matched(b_score) => a_score.cmp(&b_score),
            },
        })
        .copied()
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

fn check_yams(dice: &Dice) -> CombinationResult {
    if is_yams(dice) {
        CombinationResult::Matched(50)
    } else {
        CombinationResult::NotMatched
    }
}

fn is_yams(dice: &Dice) -> bool {
    dice.iter().all(|&die| die == dice[0])
}

fn check_chance(dice: &Dice) -> CombinationResult {
    let sum: u8 = dice.iter().sum();
    CombinationResult::Matched(sum as u32)
}

fn calculate_chance_score(dice: &Dice) -> u32 {
    let sum: u8 = dice.iter().sum();
    sum as u32
}

#[cfg(test)]
mod test {
    use super::{Combination, ORDERED_COMBINATIONS, calculate_yams_round_score};

    #[test]
    fn test_three_of_a_kind() {
        let dice = [3, 3, 3, 2, 5];
        assert_eq!(
            calculate_yams_round_score(&dice, &ORDERED_COMBINATIONS),
            Some((Combination::ThreeOfAKind, 28)),
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let dice = [4, 4, 4, 4, 1];
        assert_eq!(
            calculate_yams_round_score(&dice, &ORDERED_COMBINATIONS),
            Some((Combination::FourOfAKind, 35)),
        );
    }

    #[test]
    fn test_full_house() {
        let dice = [2, 2, 3, 3, 3];
        assert_eq!(
            calculate_yams_round_score(&dice, &ORDERED_COMBINATIONS),
            Some((Combination::FullHouse, 30)),
        );
    }

    #[test]
    fn test_straight() {
        let dice = [1, 2, 3, 4, 5];
        assert_eq!(
            calculate_yams_round_score(&dice, &ORDERED_COMBINATIONS),
            Some((Combination::Straight, 40)),
        );
    }

    #[test]
    fn test_yams() {
        let dice = [6, 6, 6, 6, 6];
        assert_eq!(
            calculate_yams_round_score(&dice, &ORDERED_COMBINATIONS),
            Some((Combination::Yams, 50)),
        );
    }

    #[test]
    fn test_chance() {
        let dice = [1, 2, 3, 4, 6];
        assert_eq!(
            calculate_yams_round_score(&dice, &ORDERED_COMBINATIONS),
            Some((Combination::Chance, 16)),
        );
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

    #[test]
    fn test_no_repeat_combination() {
        let rounds = vec![
            [1, 1, 1, 2, 2], // Full house: 30
            [2, 2, 2, 3, 3], // Three of a kind: 28
            [3, 3, 3, 4, 4], // Chance: 17
        ];
    }
}
