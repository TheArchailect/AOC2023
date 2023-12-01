use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"\d").unwrap();
    let mut pairs = Vec::new();

    for line in input.lines() {
        let base10: Vec<u32> = regex
            .find_iter(line)
            .filter_map(|mat| mat.as_str().parse::<u32>().ok())
            .collect();

        match base10.len() {
            0 => continue,
            1 => pairs.push((base10[0], base10[0])),
            _ => pairs.push((base10[0], *base10.last().unwrap())),
        }
    }

    log_pairs(&pairs);
    return Some(sum_pairs_as_digits(&pairs));
}

fn sum_pairs_as_digits(pairs: &[(u32, u32)]) -> u32 {
    pairs
        .iter()
        .map(|(a, b)| {
            // Concatenate the digits and convert to a single number
            format!("{}{}", a, b).parse::<u32>().unwrap_or(0)
        })
        .sum()
}

fn log_pairs(pairs: &[(u32, u32)]) {
    // Debugging: print the pairs
    for (index, (first, last)) in pairs.iter().enumerate() {
        println!("base10[{}] = ({}, {})", index, first, last);
    }
 }    
 

pub fn part_two(input: &str) -> Option<u32> {
    // Map of number words to their numeric values
    let number_words = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();

    let regex = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut pairs = Vec::new();

    for line in input.lines() {
        let mut base10 = Vec::new();
        let mut start = 0;
        while let Some(mat) = regex.find(&line[start..]) {
            // Extract the matching string. The match is relative to the current 'start',
            // so we adjust the indices accordingly.
            let match_str = &line[start..][mat.start()..mat.end()];
        
            // Log the found match for debugging.
            println!("Found match: '{}'", match_str);
        
            // Check if the match is a word in the number_words map. If it is, get the corresponding number.
            // If it's not a word, try parsing it as a digit.
            if let Some(&num) = number_words.get(match_str) {
                base10.push(num);
            } else if let Ok(num) = match_str.parse::<u32>() {
                base10.push(num);
            }
        
            // Increment 'start' to continue searching the rest of the string.
            // We add the length of the first character of the match to allow for overlapping matches.
            // For example, in "oneight", this lets us find "one" and then "eight".
            start += mat.start() + match_str.chars().next().map_or(0, |_| 1);
        
            // Log the updated start position for debugging.
            println!("Next start position: {}", start);
        }
        

        match base10.len() {
            0 => continue,
            1 => pairs.push((base10[0], base10[0])),
            _ => pairs.push((base10[0], *base10.last().unwrap())),
        }
    }

    log_pairs(&pairs);
    return Some(sum_pairs_as_digits(&pairs));
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
