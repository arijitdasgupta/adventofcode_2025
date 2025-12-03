use std::io::{self, BufRead};

const MAX_LEN: u8 = 12;

fn numb_from_vec(input: &[u8]) -> u64 {
    let mut input = input.to_vec();
    input.reverse();
    let mut num: u64 = 0;
    for i in 0..input.len() {
        num += (input[i] as u64) * (10_u64.pow(i as u32))
    }

    num
}

fn find_max_num(input: Vec<u8>) -> u64 {
    assert!(
        input.len() >= MAX_LEN.into(),
        "input must be at least {MAX_LEN} elements long"
    );

    let mut digits = Vec::new();
    let mut start_idx = 0;

    for pos in 0..MAX_LEN as usize {
        let search_end = input.len() - (MAX_LEN as usize - pos);

        let mut max_digit = 0;
        let mut max_idx = start_idx;

        for idx in start_idx..=search_end {
            if input[idx] > max_digit {
                max_digit = input[idx];
                max_idx = idx;
            }
        }

        digits.push(max_digit);
        start_idx = max_idx + 1;
    }

    numb_from_vec(&digits)
}

fn main() {
    let stdin = io::stdin();
    let mut sum: u64 = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let digits: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        sum = sum + find_max_num(digits);
    }

    println!("{:?}", sum);
}

#[cfg(test)]
mod test {
    use crate::{find_max_num, numb_from_vec};

    #[test]
    fn numb_compute_1() {
        let numbs = vec![2, 3];
        assert_eq!(numb_from_vec(&numbs), 23);
    }

    #[test]
    fn numb_compute_2() {
        let numbs = vec![0, 2, 3];
        assert_eq!(numb_from_vec(&numbs), 23);
    }

    #[test]
    fn numb_compute_3() {
        let numbs = vec![1, 2, 3];
        assert_eq!(numb_from_vec(&numbs), 123);
    }

    #[test]
    fn compute_max_val_1() {
        let numbs = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(find_max_num(numbs), 987654321111);
    }

    #[test]
    fn compute_max_val_2() {
        let numbs = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(find_max_num(numbs), 811111111119);
    }

    #[test]
    fn compute_max_val_3() {
        let numbs = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(find_max_num(numbs), 434234234278);
    }

    #[test]
    fn compute_max_val_4() {
        let numbs = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(find_max_num(numbs), 888911112111);
    }
}
