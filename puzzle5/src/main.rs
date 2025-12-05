use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    if let Some(first_part) = parts.get(0) {
        for line in first_part.lines() {
            let range_parts: Vec<&str> = line.split('-').collect();
            if range_parts.len() == 2 {
                let start: u64 = range_parts[0]
                    .trim()
                    .parse()
                    .expect("Failed to parse range start");
                let end: u64 = range_parts[1]
                    .trim()
                    .parse()
                    .expect("Failed to parse range end");
                ranges.push((start, end));
            }
        }
    }

    let mut numbers: Vec<u64> = Vec::new();
    if let Some(second_part) = parts.get(1) {
        for line in second_part.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let num: u64 = trimmed.parse().expect("Failed to parse number");
                numbers.push(num);
            }
        }
    }

    let mut count = 0;
    for n in numbers.iter() {
        for (min, max) in ranges.iter() {
            if n >= min && n <= max {
                count += 1;
                break;
            }
        }
    }

    println!("Parsed {} ranges", ranges.len());
    println!("Parsed {} numbers", numbers.len());
    println!("Count {count}");
}
