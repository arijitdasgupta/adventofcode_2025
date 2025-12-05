use std::io::{self, Read};

fn count_ranges(ranges: Vec<(u64, u64)>) -> u64 {
    let mut count = 0;

    for i in 0..ranges.len() {
        let (current_min, current_max) = ranges[i];
        assert!(current_min < current_max, "min should be lower than max");

        count += current_max - current_min + 1;
    }

    count
}

fn squash_and_sort_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|&(start, _)| start);

    let mut new_ranges = vec![];

    let (first_min, first_max) = ranges[0];

    new_ranges.push((first_min, first_max));

    for i in 1..ranges.len() {
        let (current_min, current_max) = ranges[i];
        let last_inserted_idx = new_ranges.len() - 1;
        let (prev_min, prev_max) = new_ranges[last_inserted_idx];

        if current_min <= prev_max + 1 {
            new_ranges[last_inserted_idx] = (prev_min, current_max.max(prev_max));
        } else {
            new_ranges.push((current_min, current_max));
        }
    }

    new_ranges
}

fn count(ranges: Vec<(u64, u64)>) -> u64 {
    let new_ranges = squash_and_sort_ranges(ranges.clone());
    let count = count_ranges(new_ranges);
    count
}

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

    let count = count(ranges.clone());

    println!("Parsed {} ranges", ranges.len());
    println!("Count {count}");
}

#[cfg(test)]
mod test {
    use crate::{count, squash_and_sort_ranges};

    #[test]
    fn squash_and_sort_1() {
        let ranges = vec![(1, 3), (4, 5)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 5)]);
    }

    #[test]
    fn squash_and_sort_2() {
        let ranges = vec![(1, 6), (3, 10)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 10)]);
    }

    #[test]
    fn squash_and_sort_3() {
        let ranges = vec![(1, 10), (2, 8)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 10)]);
    }

    #[test]
    fn squash_and_sort_4() {
        let ranges = vec![(1, 10), (10, 13)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 13)]);
    }

    #[test]
    fn squash_and_sort_5() {
        let ranges = vec![(1, 10), (1, 8)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 10)]);
    }

    #[test]
    fn squash_and_sort_6() {
        let ranges = vec![(1, 10), (11, 12)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 12)]);
    }

    #[test]
    fn squash_and_sort_7() {
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(3, 5), (10, 20)]);
    }

    #[test]
    fn squash_and_sort_8() {
        let ranges = vec![(1, 10), (8, 8)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 10)]);
    }

    #[test]
    fn squash_and_sort_9() {
        let ranges = vec![(1, 5), (4, 6)];
        assert_eq!(squash_and_sort_ranges(ranges), vec![(1, 6)]);
    }

    #[test]
    fn count_range_1() {
        let ranges = vec![(1, 3), (4, 5)];
        assert_eq!(count(ranges), 5);
    }

    #[test]
    fn count_range_2() {
        let ranges = vec![(1, 5), (4, 6)];
        assert_eq!(count(ranges), 6);
    }

    #[test]
    fn count_range_3() {
        let ranges = vec![(4, 6), (1, 5)];
        assert_eq!(count(ranges), 6);
    }

    #[test]
    fn count_range_4() {
        let ranges = vec![(10, 15), (13, 15)];
        assert_eq!(count(ranges), 6);
    }

    #[test]
    fn count_range_5() {
        let ranges = vec![(10, 15), (10, 14)];
        assert_eq!(count(ranges), 6);
    }

    #[test]
    fn count_range_6() {
        let ranges = vec![(10, 15), (12, 17)];
        assert_eq!(count(ranges), 8);
    }

    #[test]
    fn count_range_7() {
        let ranges = vec![(10, 15), (12, 14)];
        assert_eq!(count(ranges), 6);
    }

    #[test]
    fn count_range_8() {
        let ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        assert_eq!(count(ranges), 14);
    }
}
