advent_of_code::solution!(6);
struct Race {
    race_max_time:u32,
    race_distance_record: u64,
    race_record_speed: u32,
    max_possible_speed: Option<u32>,
    max_possible_distance: u64,
}

fn calculate_maximum_distance(race_duration: u64) -> u64 {
    if race_duration == 0 {
        return 0;
    }

    let penalty_time = race_duration / 2;
    let distance1 = penalty_time.checked_mul(race_duration.checked_sub(penalty_time).unwrap_or(0)).unwrap_or(u64::MAX);

    // Check the adjacent value as well
    let penalty_time_adjacent = if race_duration % 2 == 0 { penalty_time.checked_sub(1).unwrap_or(0) } else { penalty_time.checked_add(1).unwrap_or(u64::MAX) };
    let distance2 = penalty_time_adjacent.checked_mul(race_duration.checked_sub(penalty_time_adjacent).unwrap_or(0)).unwrap_or(u64::MAX);

    std::cmp::max(distance1, distance2)
}

pub fn part_one(_input: &str) -> Option<u32> {
    let mut lines = _input.lines();

    let times: Vec<u32> = lines
        .next()
        .unwrap_or("")
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap_or("")
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();    
    
    let races: Vec<Race> = times
        .into_iter()
        .zip(distances.into_iter())
        .filter_map(|(race_duration, distance)| {
            calculate_penalty_time(race_duration, distance).map(|record_speed| {
                let max_possible_distance = calculate_maximum_distance(race_duration as u64);
                let max_possible_speed = calculate_penalty_time(race_duration, max_possible_distance);

                Race {
                    race_max_time: race_duration, 
                    race_distance_record: distance,
                    race_record_speed: record_speed,
                    max_possible_distance, 
                    max_possible_speed
                }
            })
        })
    .collect();

let mut sum: u32 = 1;
for race in races {
        let mut valid_speed_options_count = 0;
        println!("~ Analyzing Race: Record calculated speed: {}, Max possible distance: {}, Max possible speed: {:?}", 
                 race.race_record_speed, 
                 race.max_possible_distance, 
                 race.max_possible_speed);
    
        for penalty_time in 0..race.race_max_time {
            let speed = penalty_time;
            let distance = (speed * (race.race_max_time - penalty_time)) as u64;
    
           println!("Penalty Time: {}, Speed: {}, Distance: {}", penalty_time, speed, distance);
    
            if distance > race.race_distance_record {
                valid_speed_options_count += 1;
                println!("Valid option found: Penalty Time: {}, Speed: {}, Distance: {}", penalty_time, speed, distance);
            }
        }
        sum *= valid_speed_options_count;
    }
    
    println!("Total valid option sum: {}", sum);
    Some(sum)
    
    
}

fn calculate_penalty_time(race_duration: u32, distance: u64) -> Option<u32> {
    let a: i64 = -1;
    let b: i64 = race_duration as i64;
    let c: i64 = -(distance as i64);

    let discriminant = b.pow(2) - 4 * a * c;

    if discriminant < 0 {
        None
    } else {
        // Calculate both possible solutions
        let sqrt_discriminant = (discriminant as f64).sqrt();
        let penalty_time1 = ((-b as f64) + sqrt_discriminant) / (2.0 * a as f64);
        let penalty_time2 = ((-b as f64) - sqrt_discriminant) / (2.0 * a as f64);

        let valid_penalty_times: Vec<u32> = [penalty_time1, penalty_time2]
            .iter()
            .filter_map(|&time| {
                let time = time.round() as i64;
                if time >= 0 && time <= race_duration as i64 {
                    Some(time as u32)
                } else {
                    None
                }
            })
            .collect();
        valid_penalty_times.into_iter().max()
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut lines = _input.lines();

    let times: Vec<u32> = lines
        .next()
        .unwrap_or("")
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let distances: Vec<u32> = lines
        .next()
        .unwrap_or("")
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();    

        let time_str = times.iter().map(|&num| num.to_string()).collect::<String>();
        let race_duration: u64 = time_str.parse().unwrap_or(0);

        let distance_str = distances.iter().map(|&num| num.to_string()).collect::<String>();
        let distance_record: u64 = distance_str.parse().unwrap_or(0);
        
        println!("Race Duration: {}, Distance Record: {}", race_duration, distance_record);

        let record_speed = calculate_penalty_time(race_duration as u32, distance_record);
        let max_possible_distance = calculate_maximum_distance(race_duration);
        let max_possible_speed = calculate_penalty_time(race_duration as u32, max_possible_distance);
        
        let race = Race {
            race_max_time: race_duration as u32, 
            race_distance_record: distance_record,
            race_record_speed: record_speed.unwrap_or(0),
            max_possible_distance, 
            max_possible_speed
        };

    let mut valid_speed_options_count = 0;
    
    println!("~ Analyzing Race: Record calculated speed: {}, Max possible distance: {}, Max possible speed: {:?}", 
             race.race_record_speed, 
             race.max_possible_distance, 
             race.max_possible_speed);


    for penalty_time in 0..race.race_max_time {
        let speed: u64 = penalty_time.into();
        let distance: u64 = speed * (race.race_max_time as u64 - penalty_time as u64);

        if distance > race.race_distance_record.into() {
            valid_speed_options_count += 1;
        }
    }


    Some(valid_speed_options_count)
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
