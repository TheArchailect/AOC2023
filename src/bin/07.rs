use std::collections::HashMap;
advent_of_code::solution!(7);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_value(card: char, card_values: [u8; 13]) -> u8 {
    match card {
        'A' => card_values[0],
        'K' => card_values[1],
        'Q' => card_values[2],
        'J' => card_values[3],
        'T' => card_values[4],
        '9' => card_values[5],
        '8' => card_values[6],
        '7' => card_values[7],
        '6' => card_values[8],
        '5' => card_values[9],
        '4' => card_values[10],
        '3' => card_values[11],
        '2' => card_values[12],
        _ => 0,
    }
}

fn wrangle_hand(hand: &str, card_values: [u8; 13]) -> (HandRank, [u8; 5]) {
    let mut counts = HashMap::new();
    let card_values_vec: Vec<u8> = hand.chars().map(|c| card_value(c, card_values)).collect();

    // Convert Vec<u8> to [u8; 5]
    let card_tuple = match card_values_vec.try_into() {
        Ok(arr) => arr,
        Err(_) => panic!("Hand does not have 5 cards"),
    };
    for card in hand.chars() {
        let value = card_value(card, card_values);
        *counts.entry(value).or_insert(0) += 1;
    }

    let mut pair_count = 0;
    let mut three_count = 0;
    let mut four_count = 0;
    let mut five_count = 0;

    for &count in counts.values() {
        match count {
            2 => pair_count += 1,
            3 => three_count += 1,
            4 => four_count += 1,
            5 => five_count += 1,
            _ => {}
        }
    }

    match (five_count, four_count, three_count, pair_count) {
        (1, _, _, _) => (HandRank::FiveOfAKind, card_tuple),
        (_, 1, _, _) => (HandRank::FourOfAKind, card_tuple),
        (_, _, 1, 1) => (HandRank::FullHouse, card_tuple),
        (_, _, 1, _) => (HandRank::ThreeOfAKind, card_tuple),
        (_, _, _, 2) => (HandRank::TwoPair, card_tuple),
        (_, _, _, 1) => (HandRank::OnePair, card_tuple),
        _ => (HandRank::HighCard, card_tuple),
    }
}

fn log_game(game: Vec<(HandRank, [u8; 5], u32)>) {
    for (index, hand) in game.iter().enumerate() {
        let rank = &hand.0;
        let color = hand_rank_color(&rank);
        let winnings = (index as u32 + 1) * hand.2;
        println!(
            "{}: {}{:?} - Hand: {:?} - Bet: {:?} - Winnings: {}{} ",
            index, color, rank, hand.1, hand.2, winnings, "\x1b[0m"
        );
    }
}

// for console debugging with colour coding
fn hand_rank_color(rank: &HandRank) -> &str {
    match rank {
        HandRank::FiveOfAKind => "\x1b[35m",  // Magenta
        HandRank::FourOfAKind => "\x1b[34m",  // Blue
        HandRank::FullHouse => "\x1b[32m",    // Green
        HandRank::ThreeOfAKind => "\x1b[36m", // Cyan
        HandRank::TwoPair => "\x1b[33m",      // Yellow
        HandRank::OnePair => "\x1b[31m",      // Red
        HandRank::HighCard => "\x1b[37m",     // White
    }
}

fn process_game_input(
    game: &str,
    card_values: [u8; 13],
    use_wildcard: bool,
) -> Vec<(HandRank, [u8; 5], u32)> {
    let mut hands = Vec::new();
    for line in game.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(bet) = parts[1].parse::<u32>() {
                hands.push((parts[0].to_string(), bet));
            }
        }
    }

    let mut tuple_hands: Vec<(HandRank, [u8; 5], u32)> = Vec::new();

    for hand in hands {
        let (original_hand_rank, original_hand_cards) = wrangle_hand(&hand.0, card_values);
        let mut best_rank = original_hand_rank;

        if use_wildcard {
            let card_symbols = [
                "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
            ];

            let has_wildcard = hand.0.contains('J');

            if has_wildcard {
                for wildcard_value in card_symbols {
                    let modified_hand = hand.0.replace('J', &wildcard_value);
                    let (rank, _) = wrangle_hand(&modified_hand, card_values);

                    if rank > best_rank {
                        best_rank = rank;
                    }
                }
            }
        }

        // store the rank / card tuple (as the originals regardless of wildcard replacements) / bet value
        tuple_hands.push((best_rank, original_hand_cards, hand.1));
    }

    tuple_hands
}

pub fn part_one(input: &str) -> Option<u32> {
    const CARD_VALUES: [u8; 13] = [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut games_hands = process_game_input(input, CARD_VALUES, false);
    games_hands.sort();
    let mut sum: u32 = 0;

    for (index, hand) in games_hands.iter().enumerate() {
        let winnings = (index as u32 + 1) * hand.2;
        sum += winnings;
    }
    // log_game(games_hands);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    const CARD_VALUES: [u8; 13] = [14, 13, 12, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut games_hands = process_game_input(input, CARD_VALUES, true);
    games_hands.sort();
    let mut sum: u32 = 0;

    for (index, hand) in games_hands.iter().enumerate() {
        let winnings = (index as u32 + 1) * hand.2;
        sum += winnings;
    }

    // log_game(games_hands);
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}