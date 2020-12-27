use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use fxhash::{FxHashSet, FxHasher64};

#[derive(PartialEq)]
enum Player {
    Me,
    Crab,
}

fn parse(input: &Vec<&str>) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut player_1_cards = VecDeque::new();
    let mut player_2_cards = VecDeque::new();

    for (i, line) in input.iter().enumerate() {
        if i == 0 || i == input.len() / 2 {
            continue;
        }

        if i < input.len() / 2 {
            player_1_cards.push_back(line.parse::<u8>().unwrap());
        } else {
            player_2_cards.push_back(line.parse::<u8>().unwrap());
        }
    }

    (player_1_cards, player_2_cards)
}

fn winner_cards<'a>(
    player_1_cards: &'a VecDeque<u8>,
    player_2_cards: &'a VecDeque<u8>,
) -> &'a VecDeque<u8> {
    if player_1_cards.is_empty() {
        player_2_cards
    } else {
        player_1_cards
    }
}

fn calc_points(winner_cards: &VecDeque<u8>) -> usize {
    winner_cards
        .iter()
        .rev()
        .enumerate()
        .map(|(index, card)| (index + 1) * *card as usize)
        .sum()
}

fn hash(player_1_cards: &VecDeque<u8>, player_2_cards: &VecDeque<u8>) -> u64 {
    let mut hasher = FxHasher64::default();

    player_1_cards.hash(&mut hasher);
    player_2_cards.hash(&mut hasher);

    hasher.finish()
}

fn play_v1(player_1_cards: &mut VecDeque<u8>, player_2_cards: &mut VecDeque<u8>) {
    loop {
        if player_1_cards.is_empty() || player_2_cards.is_empty() {
            break;
        }

        let card_1 = player_1_cards.pop_front().unwrap();
        let card_2 = player_2_cards.pop_front().unwrap();

        if card_1 > card_2 {
            player_1_cards.push_back(card_1);
            player_1_cards.push_back(card_2);
        } else {
            player_2_cards.push_back(card_2);
            player_2_cards.push_back(card_1);
        }
    }
}

fn play_v2(player_1_cards: &mut VecDeque<u8>, player_2_cards: &mut VecDeque<u8>) -> Player {
    let mut seen = FxHashSet::default();

    while false == player_1_cards.is_empty() && false == player_2_cards.is_empty() {
        if false == seen.insert(hash(player_1_cards, player_2_cards)) {
            return Player::Me;
        }

        let card_1 = player_1_cards.pop_front().unwrap() as usize;
        let card_2 = player_2_cards.pop_front().unwrap() as usize;

        let winner = if card_1 <= player_1_cards.len() && card_2 <= player_2_cards.len() {
            let mut player_1_cards_clone = player_1_cards.iter().take(card_1).copied().collect();
            let mut player_2_cards_clone = player_2_cards.iter().take(card_2).copied().collect();

            play_v2(&mut player_1_cards_clone, &mut player_2_cards_clone)
        } else {
            if card_1 > card_2 {
                Player::Me
            } else {
                Player::Crab
            }
        };

        if winner == Player::Me {
            player_1_cards.push_back(card_1 as u8);
            player_1_cards.push_back(card_2 as u8);
        } else {
            player_2_cards.push_back(card_2 as u8);
            player_2_cards.push_back(card_1 as u8);
        }
    }

    if player_1_cards.is_empty() {
        Player::Crab
    } else {
        Player::Me
    }
}

pub fn part1(input: &Vec<&str>) -> usize {
    let (mut player_1_cards, mut player_2_cards) = parse(input);

    play_v1(&mut player_1_cards, &mut player_2_cards);
    calc_points(winner_cards(&player_1_cards, &player_2_cards))
}

pub fn part2(input: &Vec<&str>) -> usize {
    let (mut player_1_cards, mut player_2_cards) = parse(input);

    play_v2(&mut player_1_cards, &mut player_2_cards);
    calc_points(winner_cards(&player_1_cards, &player_2_cards))
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 306)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 291)
    }
}
