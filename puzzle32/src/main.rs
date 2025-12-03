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

fn compute_number(input: &[u8], indices: &[usize]) -> u64 {
    let mut final_numb_vec = vec![];
    for i in indices.iter() {
        final_numb_vec.push(input[*i as usize]);
    }

    numb_from_vec(&final_numb_vec)
}

fn generate_increasing(
    length: usize,
    max_val: usize,
    current: &mut Vec<usize>,
    results: &mut Vec<Vec<usize>>,
) {
    if current.len() == length {
        results.push(current.clone());
        return;
    }
    let min_next = current.last().map(|&v| v + 1).unwrap_or(0);
    for val in min_next..=max_val {
        current.push(val);
        generate_increasing(length, max_val, current, results);
        current.pop();
    }
}

fn find_max_num(input: Vec<u8>) -> u64 {
    assert!(
        input.len() >= MAX_LEN.into(),
        "input must be at least {MAX_LEN} elements long"
    );

    let mut current_idxs: Vec<usize> = (0..MAX_LEN as usize).collect();
    let mut all_idx_combos: Vec<Vec<usize>> = vec![];

    let mut max_val = 0;

    generate_increasing(
        MAX_LEN as usize,
        input.len() - 1,
        &mut current_idxs,
        &mut all_idx_combos,
    );

    for combo in all_idx_combos {
        let number = compute_number(&input, combo.as_slice());
        if number > max_val {
            max_val = number;
        }
    }

    return max_val.into();
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

        sum = sum + (find_max_num(digits) as u32);
    }

    println!("{:?}", sum);
}

#[cfg(test)]
mod test {
    use crate::{compute_number, find_max_num, numb_from_vec};

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
    fn computer_number_1() {
        let numbs = vec![0, 2, 3];
        let indices = vec![1, 2];
        assert_eq!(compute_number(&numbs, &indices), 23);
    }

    #[test]
    fn computer_number_2() {
        let numbs = vec![0, 2, 3];
        let indices = vec![0, 2];
        assert_eq!(compute_number(&numbs, &indices), 3);
    }

    #[test]
    fn computer_number_3() {
        let numbs = vec![1, 2, 3, 4, 5, 6, 7];
        let indices = vec![1, 2, 4];
        assert_eq!(compute_number(&numbs, &indices), 235);
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
