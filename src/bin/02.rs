use std::collections::HashMap;

advent_of_code::solution!(2);
// Define a type alias for a HashMap where the key is a String (representing a tile colour) and the value is a u32 (representing the number of colour drawn from a bag).
type ColourCounts = HashMap<String, u32>;

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the input string into a games map.
    let games_map = parse_games(input);

    // Define a HashMap with maximum counts for each colour (representing the restriction for possible games).
    let max_counts = HashMap::from([
        ("red".to_string(), 12),
        ("blue".to_string(), 14),
        ("green".to_string(), 13),
    ]);

    // Filter the games based on the maximum counts defined above.
    let filtered_games = filter_games_by_max_counts(&games_map, &max_counts);

    // Calculate the sum of the filtered game numbers using iterator methods.
    let sum: u32 = filtered_games.iter().sum();

    // Return the sum wrapped in Some, a Rust enum for handling optional values.
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games_map = parse_games(input);
    return Some(sum_power(&games_map));
}

// Function to parse the input string into a HashMap.
fn parse_games(input: &str) -> HashMap<u32, Vec<ColourCounts>> {
    input
        .lines()
        .filter_map(|line| {
            // Rust's pattern matching
            let mut parts = line.split(':');
            let game_number = parts
                .next()?
                .trim()
                .split_whitespace()
                .nth(1)?
                .parse()
                .ok()?;
            let subsets = parts
                .next()?
                .trim()
                .split(';')
                .map(|subset| {
                    subset
                        .split(',')
                        .filter_map(|colour_count| {
                            let mut parts = colour_count.trim().split_whitespace();
                            let count = parts.next()?.parse().ok()?;
                            let colour = parts.next()?.to_string();
                            Some((colour, count))
                        })
                        .collect()
                })
                .collect();
            Some((game_number, subsets))
        })
        .collect()
}

fn filter_games_by_max_counts(
    games_map: &HashMap<u32, Vec<ColourCounts>>,
    max_counts: &HashMap<String, u32>,
) -> Vec<u32> {
    games_map
        .iter()
        .filter(|(_, subsets)| {
            subsets.iter().all(|subset| {
                subset.iter().all(|(colour, &count)| {
                    max_counts
                        .get(colour)
                        .map_or(true, |&max_count| count <= max_count)
                })
            })
        })
        .map(|(&game, _)| game)
        .collect()
}

// Function to calculate the sum of powers of the games.
fn sum_power(games_map: &HashMap<u32, Vec<ColourCounts>>) -> u32 {
    let mut min_colours_per_game = HashMap::new();

    for (&game_number, subsets) in games_map {
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for subset in subsets {
            for (colour, &count) in subset {
                match colour.as_str() {
                    "red" => max_red = max_red.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    "green" => max_green = max_green.max(count),
                    _ => {}
                }
            }
        }

        min_colours_per_game.insert(
            game_number,
            HashMap::from([
                ("red".to_string(), max_red),
                ("blue".to_string(), max_blue),
                ("green".to_string(), max_green),
            ]),
        );
    }

    min_colours_per_game
        .values()
        .map(|colours| colours.values().product::<u32>())
        .sum()
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
