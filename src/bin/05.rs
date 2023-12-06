use std::collections::HashMap;
use std::io::Write;
use std::sync::{Mutex, Arc};
use rayon::prelude::*;
advent_of_code::solution!(5);

struct LabeledMap {
    map: HashMap<Mapping, Mapping>,
    source_stage: Stage,
    destination_stage: Stage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Stage {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Mapping {
    source_stage: Stage,
    destination_stage: Stage,
    source_start: u64, 
    source_end: u64,  
    destination_start: u64, 
}

// struct Range {
//     start: u64,
//     end: u64,
// }

fn build_map(
    input: &str,
    source_stage: Stage,
    destination_stage: Stage,
) -> HashMap<Mapping, Mapping> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let parts: Vec<u64> = line 
            .split_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();
        if parts.len() == 3 {
            let (destination_start, source_start, length) = (parts[0], parts[1], parts[2]);
            let source = Mapping {
                source_stage,
                destination_stage,
                source_start,
                source_end: source_start + length as u64 - 1, 
                destination_start,
            };
            map.insert(source.clone(), source);
        }
    }

    map
}

const SEED_TO_SOIL: &str = "seed-to-soil map";
const SOIL_TO_FERTILIZER: &str = "soil-to-fertilizer map";
const FERTILIZER_TO_WATER: &str = "fertilizer-to-water map";
const WATER_TO_LIGHT: &str = "water-to-light map";
const LIGHT_TO_TEMPERATURE: &str = "light-to-temperature map";
const TEMPERATURE_TO_HUMIDITY: &str = "temperature-to-humidity map";
const HUMIDITY_TO_LOCATION: &str = "humidity-to-location map";

fn build_all_maps(input: &str) -> (Vec<u32>, Vec<LabeledMap>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<u32> = sections[0]
        .split(": ")
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .filter_map(|num| num.parse::<u32>().ok())
        .collect();

    let mut labeled_maps: Vec<LabeledMap> = Vec::new();

    let map_order = vec![
        (SEED_TO_SOIL, Stage::Seed, Stage::Soil),
        (SOIL_TO_FERTILIZER, Stage::Soil, Stage::Fertilizer),
        (FERTILIZER_TO_WATER, Stage::Fertilizer, Stage::Water),
        (WATER_TO_LIGHT, Stage::Water, Stage::Light),
        (LIGHT_TO_TEMPERATURE, Stage::Light, Stage::Temperature),
        (TEMPERATURE_TO_HUMIDITY, Stage::Temperature, Stage::Humidity),
        (HUMIDITY_TO_LOCATION, Stage::Humidity, Stage::Location),
    ];

    for (map_name, source_stage, destination_stage) in map_order {
        if let Some(section) = sections.iter().find(|&s| s.starts_with(map_name)) {
            let map = build_map(section, source_stage, destination_stage);
            labeled_maps.push(LabeledMap{
                map, 
                source_stage: source_stage,
                destination_stage: destination_stage,
            });
        }
    }

    (seeds, labeled_maps)
}

fn traverse(maps: &[LabeledMap], start_value: u64, start_stage: Stage, end_stage: Stage) -> Option<u64> {
    let mut current_value = start_value;
    let mut current_stage = start_stage;

    while current_stage != end_stage {
        if let Some(labeled_map) = maps.iter().find(|map| map.source_stage == current_stage) {
            let mapping = labeled_map.map.values().find(|&m| 
                m.source_start <= current_value && current_value <= m.source_end);

            match mapping {
                Some(m) => {
                    let offset = current_value - m.source_start;
                    current_value = m.destination_start + offset;
                },
                None => {
                    // If no mapping is found, assume the value remains the same
                }
            }

            current_stage = labeled_map.destination_stage;
        } else {
            return None; // No map found for the current stage
        }
    }

    Some(current_value)
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (seeds, _maps) = build_all_maps(_input);

    println!("Starting to process seeds: {:?}", seeds);

    let mut locations = Vec::new();

    for seed in seeds {
        println!("Processing seed: {}", seed);
        if let Some(location) = traverse(&_maps, seed as u64, Stage::Seed, Stage::Location) {
            println!("Seed {} -> Location {}", seed, location);
            locations.push(location);
        } else {
            println!("Traversal failed for seed {}", seed);
        }
    }

    let min_location = locations.into_iter().min();
    println!("Minimum location found: {:?}", min_location);

    Some(1)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (seeds, maps) = build_all_maps(_input);

    if seeds.len() % 2 != 0 {
        println!("Seeds vector does not contain a valid number of elements for pairs.");
        return None;
    }

    let maps = Arc::new(maps);
    let total_length: u64 = seeds.chunks(2).map(|pair| pair[1] as u64).sum();
    let processed_length = Arc::new(Mutex::new(0u64));

    println!("Starting to process seed pairs.");

    let all_locations: Vec<u32> = seeds
        .par_chunks(2)
        .flat_map(|seed_pair| {
            let maps = maps.clone();
            let processed_length = processed_length.clone();

            if let [start, length] = *seed_pair {
                (0..length).into_par_iter().filter_map(move |i| {
                    let seed = start + i;
                    if let Some(location) = traverse(&maps, seed as u64, Stage::Seed, Stage::Location) {
                        let mut processed = processed_length.lock().unwrap();
                        *processed += 1;

                        if *processed % 10000 == 0 {
                            print!("\rProgress: {:.2}%", (*processed as f64 / total_length as f64) * 100.0);
                            std::io::stdout().flush().unwrap();
                        }

                        Some(location as u32) 
                    } else {
                        None
                    }
                }).collect::<Vec<u32>>()
            } else {
                vec![]
            }
        })
        .collect();

    println!("\nProcessing complete.");

    let result = all_locations.into_iter().min();
    println!("Result found: {:?}", result);

    result
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
