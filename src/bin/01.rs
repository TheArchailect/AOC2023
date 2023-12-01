use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(1);

// Part one of the solution
pub fn part_one(input: &str) -> Option<u32> {
    // Compile a regular expression to find digits
    let regex = Regex::new(r"\d").unwrap();
    let mut pairs = Vec::new();

    // Iterate over each line in the input
    for line in input.lines() {
        // Create an iterator that finds all matches of the regex in the line
        // and attempts to parse each match as a u32
        let mut iter = regex
            .find_iter(line)
            .filter_map(|mat| mat.as_str().parse::<u32>().ok());

        // Determine the first and last numbers in the line
        match (iter.next(), iter.last()) {
            // If there's only one number, or no numbers, use it for both first and last
            (Some(first), None) | (None, Some(first)) => pairs.push((first, first)),
            // If there are at least two numbers, use the first and last ones
            (Some(first), Some(last)) => pairs.push((first, last)),
            // If there are no numbers, skip this line
            _ => continue,
        }
    }

    // Calculate and return the sum of all pairs
    Some(sum_pairs_as_digits(&pairs))
}

// Custom iterator to handle overlapping number matches in a string
struct NumberIterator<'a> {
    line: &'a str,
    regex: &'a Regex,
    number_words: &'a HashMap<&'a str, u32>,
    start: usize,
}

impl<'a> NumberIterator<'a> {
    // Constructor for the iterator
    fn new(line: &'a str, regex: &'a Regex, number_words: &'a HashMap<&'a str, u32>) -> Self {
        NumberIterator {
            line,
            regex,
            number_words,
            start: 0,
        }
    }
}

impl<'a> Iterator for NumberIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Attempt to find the next match in the line, starting from 'start'
        if let Some(mat) = self.regex.find(&self.line[self.start..]) {
            // Extract the matched string
            let match_str = &self.line[self.start..][mat.start()..mat.end()];
            // Update 'start' to allow for overlapping matches
            self.start += mat.start() + match_str.chars().next().map_or(0, |_| 1);

            // Convert the match to a number, either by looking it up in the map or parsing it
            if let Some(&num) = self.number_words.get(match_str) {
                Some(num)
            } else {
                match_str.parse().ok()
            }
        } else {
            // No more matches
            None
        }
    }
}

// Part two of the solution
pub fn part_two(input: &str) -> Option<u32> {
    // Map of number words to their numeric values
    let number_words: HashMap<_, _> = [
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
    .collect();

    // Compile a regular expression to find digits and number words
    let regex = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut pairs = Vec::new();

    // Iterate over each line in the input
    for line in input.lines() {
        // Create a custom iterator to find all numbers in the line
        let mut iter = NumberIterator::new(line, &regex, &number_words);

        // Determine the first and last numbers in the line
        match (iter.next(), iter.last()) {
            // If there's only one number, or no numbers, use it for both first and last
            (Some(first), None) | (None, Some(first)) => pairs.push((first, first)),
            // If there are at least two numbers, use the first and last ones
            (Some(first), Some(last)) => pairs.push((first, last)),
            // If there are no numbers, skip this line
            _ => continue,
        }
    }

    // Calculate and return the sum of all pairs
    Some(sum_pairs_as_digits(&pairs))
}

// Function to sum the pairs of numbers as concatenated digits
fn sum_pairs_as_digits(pairs: &[(u32, u32)]) -> u32 {
    pairs
        .iter()
        .map(|(a, b)| {
            // Concatenate the digits and convert to a single number
            format!("{}{}", a, b).parse::<u32>().unwrap_or(0)
        })
        .sum()
}

// Function for debugging: prints the pairs of numbers
fn _log_pairs(pairs: &[(u32, u32)]) {
    for (index, (first, last)) in pairs.iter().enumerate() {
        println!("Pair[{}] = ({}, {})", index, first, last);
    }
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
