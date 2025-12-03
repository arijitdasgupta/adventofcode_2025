use std::io::{self, BufRead};

fn find_max_pair(input: Vec<u8>) -> u8 {
    assert!(input.len() >= 1, "input must be at least two elements long");
    let mut first_idx = 0;
    let mut last_idx = 1;
    let mut max_val = 0;

    for f_idx in first_idx..input.len() - 1 {
        for l_idx in last_idx..input.len() {
            let num = input[f_idx] * 10 + input[l_idx];
            if num > max_val {
                max_val = num;
            }
        }
        first_idx += 1;
        last_idx += 1;
    }

    return max_val;
}

fn main() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let digits: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        sum = sum + (find_max_pair(digits) as u32);
    }

    println!("{:?}", sum);
}

#[cfg(test)]
mod test {
    use crate::find_max_pair;

    #[test]
    fn find_max_number_2_digit() {
        let numbs = vec![2, 3];
        assert_eq!(find_max_pair(numbs), 23);
    }

    #[test]
    fn find_max_number_3_digit() {
        let numbs = vec![2, 8, 9];
        assert_eq!(find_max_pair(numbs), 89);
    }

    #[test]
    fn find_max_number_4_digit() {
        let numbs = vec![9, 8, 7, 9];
        assert_eq!(find_max_pair(numbs), 99);
    }

    #[test]
    fn find_max_number_4_digit_2() {
        let numbs = vec![7, 8, 9, 9];
        assert_eq!(find_max_pair(numbs), 99);
    }
}
