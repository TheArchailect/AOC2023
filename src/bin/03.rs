use std::collections::HashMap;
advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Digit(u8),
    Symbol(char),
}

#[derive(Clone)]
struct Block {
    top_left: (usize, usize),
    cells: Vec<Cell>,
}

impl Block {
    fn digit_value(&self) -> u32 {
        self.cells.iter().fold(0, |acc, cell| {
            acc * 10
                + match cell {
                    Cell::Digit(d) => *d as u32,
                    _ => 0,
                }
        })
    }

    fn is_in_proximity_to_any_symbol(&self, grid: &Vec<Vec<Cell>>) -> bool {
        let (start_x, y) = self.top_left;
        let end_x = start_x + self.cells.len();

        // Check all adjacent cells including diagonals
        for x in start_x..end_x {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue; // Skip the cell itself
                    }

                    if self.is_in_proximity_to_symbol(
                        (x as isize + dx) as usize,
                        (y as isize + dy) as usize,
                        grid,
                    ) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_in_proximity_to_symbol(&self, x: usize, y: usize, grid: &Vec<Vec<Cell>>) -> bool {
        if x < grid[0].len() && y < grid.len() {
            match grid.get(y).and_then(|row| row.get(x)) {
                Some(Cell::Symbol(_)) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_in_proximity_to_point(&self, x: usize, y: usize) -> bool {
        let (start_x, block_y) = self.top_left;
        let end_x = start_x + self.cells.len();

        // Check if the point is adjacent or diagonal to any part of the block
        for block_x in start_x..end_x {
            if (x as isize - block_x as isize).abs() <= 1 && (y as isize - block_y as isize).abs() <= 1 {
                return true;
            }
        }

        false
    }
}

fn create_grid(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Cell::Empty,
                    '0'..='9' => Cell::Digit(ch.to_digit(10).unwrap() as u8),
                    _ => Cell::Symbol(ch),
                })
                .collect()
        })
        .collect()
}

fn create_blocks(grid: &Vec<Vec<Cell>>) -> Vec<Block> {
    let mut blocks = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            match row[x] {
                Cell::Digit(_) => {
                    let start_x = x;
                    while x < row.len() && matches!(row[x], Cell::Digit(_)) {
                        x += 1;
                    }
                    let block_cells = row[start_x..x].to_vec();
                    blocks.push(Block {
                        top_left: (start_x, y),
                        cells: block_cells, 
                    });
                }
                _ => x += 1,
            }
        }
    }

    blocks
}

fn print_grid_with_blocks(
    grid: &Vec<Vec<Cell>>,
    blocks: &Vec<Block>,
    filtered_blocks: &Vec<Block>,
) {
    let mut block_cells = vec![vec![false; grid[0].len()]; grid.len()];
    let mut filtered_block_cells = vec![vec![false; grid[0].len()]; grid.len()];

    // Mark all cells that are part of any block
    for block in blocks {
        let (start_x, y) = block.top_left;
        for x in start_x..start_x + block.cells.len() {
            block_cells[y][x] = true;
        }
    }

    // Mark all cells that are part of filtered blocks
    for block in filtered_blocks {
        let (start_x, y) = block.top_left;
        for x in start_x..start_x + block.cells.len() {
            filtered_block_cells[y][x] = true;
        }
    }

    // ANSI color codes
    let regular_color = "\x1b[0m"; // White
    let block_color = "\x1b[33m"; // Yellow
    let filtered_block_color = "\x1b[31m"; // Red

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                Cell::Empty => print!("."),
                Cell::Digit(d) => {
                    if filtered_block_cells[y][x] {
                        // Color filtered block cells distinctly
                        print!("{}{}{}", filtered_block_color, d, regular_color);
                    } else if block_cells[y][x] {
                        // Color other block cells
                        print!("{}{}{}", block_color, d, regular_color);
                    } else {
                        // Regular digit
                        print!("{}", d);
                    }
                }
                Cell::Symbol(s) => print!("{}", s),
            };
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = create_grid(input);
    let blocks = create_blocks(&grid);

    let filtered_blocks = blocks
        .clone()
        .into_iter()
        .filter(|block| block.is_in_proximity_to_any_symbol(&grid))
        .collect::<Vec<Block>>();

    print_grid_with_blocks(&grid, &blocks, &filtered_blocks);

    let mut sum: u64 = 0;

    for block in filtered_blocks {
        sum += block.digit_value() as u64;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = create_grid(input);
    let blocks = create_blocks(&grid);
    let mut star_block_pairs = HashMap::new();

    
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Cell::Symbol('*') =  cell {
                let mut blocks_around_star = Vec::new();

                for block in &blocks {
                    if block.is_in_proximity_to_point(x, y) {
                        blocks_around_star.push(block.clone());
                    }
                }

                if blocks_around_star.len() == 2 {
                    star_block_pairs.insert((x, y), (blocks_around_star));
                }
            }
        }
    }

    let mut sum: u64 = 0;

    for block_pair in star_block_pairs {
        sum += (block_pair.1[0].digit_value() * block_pair.1[1].digit_value()) as u64;
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
