use colored::*; // Import colored crate
advent_of_code::solution!(4);
#[derive(Clone)]
struct GameTicket {
    game_number: usize,
    winning_numbers: Vec<u32>,
    ticket_numbers: Vec<u32>,
    winning_count: usize,
}

impl GameTicket {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(": ").collect();
        if parts.len() != 2 {
            panic!("Invalid input format");
        }

        let game_number_str = parts[0]
            .trim()
            .split_whitespace()
            .last()
            .expect("Invalid game number format");
        let game_number = game_number_str
            .parse::<usize>()
            .expect("Invalid game number");

        let numbers_parts: Vec<&str> = parts[1].split('|').collect();
        
        if numbers_parts.len() != 2 {
            panic!("Invalid input format");
        }

        let winning_numbers: Vec<u32> = numbers_parts[0]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Invalid number"))
            .collect();

        let ticket_numbers: Vec<u32> = numbers_parts[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Invalid number"))
            .collect();

        // Calculate the intersection count for winning ticket numbers
        let winning_set: std::collections::HashSet<_> = winning_numbers.iter().collect();
        let winning_count = ticket_numbers
            .iter()
            .filter(|num| winning_set.contains(num))
            .count();

        GameTicket {
            game_number,
            winning_numbers,
            ticket_numbers,
            winning_count,
        }
    }

    fn calculate_winnings(&self) -> u32 {
        if self.winning_count == 0 {
            return 0; // If there are no winning numbers, return 1
        }

        let mut winnings = 1; // Start with 2 for the first winning number

        for _ in 1..self.winning_count {
            winnings *= 2; // Multiply by 2 for each additional winning number
        }

        winnings
    }

    fn print_color_coded(&self) {
        let winning_numbers_set: std::collections::HashSet<_> =
            self.winning_numbers.iter().collect();

        // Printing winning numbers
        println!(
            "Winning Numbers: {}",
            self.winning_numbers
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        let ticket_str = self
            .ticket_numbers
            .iter()
            .map(|num| {
                if winning_numbers_set.contains(num) {
                    num.to_string().red().to_string()
                } else {
                    num.to_string().normal().to_string() 
                }
            })
            .collect::<Vec<String>>()
            .join(" "); // Join the strings

        println!("Ticket Numbers: {}", ticket_str);
        let winnings = self.calculate_winnings();
        println!("Winnings: {}", winnings);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let game_tickets: Vec<GameTicket> = input.lines().map(|line| GameTicket::new(line)).collect();

    let mut winnings: u32 = 0;

    for ticket in &game_tickets {
        winnings += ticket.calculate_winnings();
        ticket.print_color_coded();
        println!();
    }

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut game_tickets: Vec<GameTicket> = input
        .lines()
        .map(|line| GameTicket::new(line))
        .collect();

    let mut instanced_tickets: Vec<GameTicket> = Vec::new();

    // Instance initial set of tickets
    for ticket in &game_tickets {
        let start = ticket.game_number + 1;
        let end = std::cmp::min(game_tickets.len(), start + ticket.winning_count);

        for idx in start..end {
            if idx - 1 <= game_tickets.len() {
                instanced_tickets.push(game_tickets[idx-1].clone());
            }
        }
    }

    // Process newly instanced tickets recursively
    let mut i = 0;
    while i < instanced_tickets.len() {
        let ticket = &instanced_tickets[i];
        let start = ticket.game_number + 1;
        let end = std::cmp::min(game_tickets.len(), start + ticket.winning_count);

        for idx in start..end {
            if idx - 1 <= game_tickets.len() {
                instanced_tickets.push(game_tickets[idx-1].clone());
            }
        }

        i += 1;
    }

    // Combine original and instanced tickets
    game_tickets.extend(instanced_tickets);

    Some(game_tickets.len() as u32)
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
