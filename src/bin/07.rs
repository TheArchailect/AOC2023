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

fn card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

fn evaluate_hand(hand: &str) -> (HandRank, [u8; 5]) {
    let mut counts = HashMap::new();
    let card_values_vec: Vec<u8> = hand.chars().map(card_value).collect();

    // Convert Vec<u8> to [u8; 5]
    let card_values = match card_values_vec.try_into() {
        Ok(arr) => arr,
        Err(_) => panic!("Hand does not have 5 cards"),
    };

    for card in hand.chars() {
        let value = card_value(card);
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
        (1, _, _, _) => (HandRank::FiveOfAKind, card_values),
        (_, 1, _, _) => (HandRank::FourOfAKind, card_values),
        (_, _, 1, 1) => (HandRank::FullHouse, card_values), 
        (_, _, 1, _) => (HandRank::ThreeOfAKind, card_values),
        (_, _, _, 2) => (HandRank::TwoPair, card_values),
        (_, _, _, 1) => (HandRank::OnePair, card_values),
        _ => (HandRank::HighCard, card_values),
    }
}

fn hand_rank_color(rank: &HandRank) -> &str {
    match rank {
        HandRank::FiveOfAKind => "\x1b[35m", // Magenta
        HandRank::FourOfAKind => "\x1b[34m", // Blue
        HandRank::FullHouse => "\x1b[32m", // Green
        HandRank::ThreeOfAKind => "\x1b[36m", // Cyan
        HandRank::TwoPair => "\x1b[33m", // Yellow
        HandRank::OnePair => "\x1b[31m", // Red
        HandRank::HighCard => "\x1b[37m", // White
    }
}

fn sort_hands(hands: Vec<(String, u32)>) -> Vec<(HandRank, [u8; 5], u32)> {
    let mut tupil_hands: Vec<(HandRank, [u8; 5], u32)> = Vec::new();
    for hand in hands {
        let hand_rank_values = evaluate_hand(&hand.0);
        tupil_hands.push((hand_rank_values.0, hand_rank_values.1, hand.1));
    }
    tupil_hands.sort();
    tupil_hands
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(bet) = parts[1].parse::<u32>() {
                hands.push((parts[0].to_string(), bet));
            }
        }
    }

    let sorted_hands = sort_hands(hands);
    let mut sum: u32 = 0;

    for (index, hand) in sorted_hands.iter().enumerate() {
        let rank = &hand.0;
        let color = hand_rank_color(&rank);
        let winnings = (index as u32 + 1) * hand.2;
        println!("{}: {}{:?} - Hand: {:?} - Bet: {:?} - Winnings: {}{} ",index, color, rank, hand.1, hand.2, winnings, "\x1b[0m");
        sum += winnings;
    }

    Some(sum)
}

fn part_two_card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        // joker wild card is now the weakest card
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

fn part_two_pre_evaluate_hand(hand: &str) -> (HandRank, [u8; 5]) {
    let (original_rank, original_hand_values) = part_two_evaluate_hand(hand);
    let has_wildcard = hand.contains('J');
    let mut best_rank = HandRank::HighCard;
    let card_values = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J"];

    if has_wildcard {
        for wildcard_value in card_values {
            let modified_hand = hand.replace('J', &wildcard_value);
            let (rank, _) = part_two_evaluate_hand(&modified_hand);
            if rank > best_rank {
                best_rank = rank;
            }
        }
    } else {
        best_rank = original_rank;
    }
    (best_rank, original_hand_values)
}

fn part_two_evaluate_hand(hand: &str) -> (HandRank, [u8; 5]) {
    let mut counts = HashMap::new();
    let card_values_vec: Vec<u8> = hand.chars().map(part_two_card_value).collect();

    // Convert Vec<u8> to [u8; 5]
    let card_values = match card_values_vec.try_into() {
        Ok(arr) => arr,
        Err(_) => panic!("Hand does not have 5 cards"),
    };

    for card in hand.chars() {
        let value = part_two_card_value(card);
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
        (1, _, _, _) => (HandRank::FiveOfAKind, card_values),
        (_, 1, _, _) => (HandRank::FourOfAKind, card_values),
        (_, _, 1, 1) => (HandRank::FullHouse, card_values), 
        (_, _, 1, _) => (HandRank::ThreeOfAKind, card_values),
        (_, _, _, 2) => (HandRank::TwoPair, card_values),
        (_, _, _, 1) => (HandRank::OnePair, card_values),
        _ => (HandRank::HighCard, card_values),
    }
}

fn part_two_sort_hands(hands: Vec<(String, u32)>) -> Vec<(HandRank, [u8; 5], u32)> {
    let mut tupil_hands: Vec<(HandRank, [u8; 5], u32)> = Vec::new();
    for hand in hands {
        let hand_rank_values = part_two_pre_evaluate_hand(&hand.0);
        tupil_hands.push((hand_rank_values.0, hand_rank_values.1, hand.1));
    }
    tupil_hands.sort();
    tupil_hands
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(bet) = parts[1].parse::<u32>() {
                hands.push((parts[0].to_string(), bet));
            }
        }
    }

    let sorted_hands = part_two_sort_hands(hands);
    let mut sum: u32 = 0;

    for (index, hand) in sorted_hands.iter().enumerate() {
        let rank = &hand.0;
        let color = hand_rank_color(&rank);
        let winnings = (index as u32 + 1) * hand.2;
        println!("{}: {}{:?} - Hand: {:?} - Bet: {:?} - Winnings: {}{} ",index, color, rank, hand.1, hand.2, winnings, "\x1b[0m");
        sum += winnings;
    }

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
