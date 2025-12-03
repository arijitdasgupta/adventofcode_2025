use std::fs;

fn digits(mut i: u64) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];
    while i > 0 {
        let digit = (i % 10) as u8;
        digits.push(digit);
        i /= 10;
    }

    digits.reverse();
    digits
}

fn is_invalid(i: u64) -> bool {
    let digits = digits(i);
    let mut window_size = 1;

    let half_len = digits.len() / 2;

    let mut last_comp: Option<usize> = None;
    while window_size <= half_len && last_comp == None {
        let mut window_idx = 0;
        let first_window = &digits[window_idx..window_idx + window_size];

        while (window_idx + window_size) <= digits.len() {
            let window = &digits[window_idx..window_idx + window_size];
            window_idx += window_size;
            if first_window != window {
                last_comp = None;
                break;
            } else {
                last_comp = Some(window_size);
            }
        }
        window_size += 1;
    }

    match last_comp {
        Some(window_size) => digits.len() % window_size == 0,
        None => false,
    }
}

fn find_invalid_ids(limit1: u64, limit2: u64) -> Vec<u64> {
    let mut invalids: Vec<u64> = vec![];

    for i in limit1..=limit2 {
        if is_invalid(i) {
            invalids.push(i);
        }
    }

    return invalids;
}

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("Failed to read file");

    let mut sum: u64 = 0;

    for item in contents.split(',') {
        let numbers: Vec<_> = item.split('-').collect();
        let limit1: u64 = numbers[0].replace("\n", "").parse().unwrap();
        let limit2: u64 = numbers[1].replace("\n", "").parse().unwrap();

        assert!(limit1 < limit2, "limit1 should be lower than limit 2");
        sum += find_invalid_ids(limit1, limit2).iter().sum::<u64>();
    }

    println!("{}", sum);
}

#[cfg(test)]
mod test {
    use crate::{digits, find_invalid_ids, is_invalid};

    // Digits
    #[test]
    fn test_digits_1() {
        let n = 345;
        assert_eq!(digits(n), vec![3, 4, 5])
    }

    #[test]
    fn test_digits_2() {
        let n = 0;
        assert_eq!(digits(n), vec![])
    }

    // Invalidity
    #[test]
    fn test_invalid_false() {
        let n: u64 = 3536363;
        assert_eq!(is_invalid(n), false);
    }

    #[test]
    fn test_invalid_true() {
        let n: u64 = 11;
        assert_eq!(is_invalid(n), true);
    }

    #[test]
    fn test_invalid_true_1() {
        let n: u64 = 1111;
        assert_eq!(is_invalid(n), true);
    }

    #[test]
    fn test_invalid_true_2() {
        let n: u64 = 112112;
        assert_eq!(is_invalid(n), true);
    }

    #[test]
    fn test_invalid_true_4() {
        let n: u64 = 111;
        assert_eq!(is_invalid(n), true);
    }

    #[test]
    fn test_invalid_true_5() {
        let n: u64 = 112112112;
        assert_eq!(is_invalid(n), true);
    }

    #[test]
    fn test_invalid_true_6() {
        let n: u64 = 112112112112;
        assert_eq!(is_invalid(n), true);
    }

    #[test]
    fn test_invalid_true_7() {
        let n: u64 = 1111111;
        assert_eq!(is_invalid(n), true);
    }

    #[test]
    fn test_invalid_false_2() {
        let n: u64 = 112211;
        assert_eq!(is_invalid(n), false);
    }

    #[test]
    fn test_invalid_false_3() {
        let n: u64 = 1120112;
        assert_eq!(is_invalid(n), false);
    }

    #[test]
    fn test_invalid_false_4() {
        let n: u64 = 112000112;
        assert_eq!(is_invalid(n), false);
    }

    #[test]
    fn test_invalid_false_6() {
        let n: u64 = 1120112;
        assert_eq!(is_invalid(n), false);
    }

    // Finder
    #[test]
    fn test_finder_1() {
        let limit1 = 11;
        let limit2 = 22;
        let invalids = find_invalid_ids(limit1, limit2);
        assert_eq!(invalids, vec![11, 22]);
    }

    #[test]
    fn test_finder_2() {
        let limit1 = 95;
        let limit2 = 115;
        let invalids = find_invalid_ids(limit1, limit2);
        assert_eq!(invalids, vec![99, 111]);
    }

    #[test]
    fn test_finder_3() {
        let limit1 = 998;
        let limit2 = 1012;
        let invalids = find_invalid_ids(limit1, limit2);
        assert_eq!(invalids, vec![999, 1010]);
    }

    #[test]
    fn test_finder_4() {
        let limit1 = 1188511880;
        let limit2 = 1188511890;
        let invalids = find_invalid_ids(limit1, limit2);
        assert_eq!(invalids, vec![1188511885]);
    }

    #[test]
    fn test_finder_5() {
        let limit1 = 1698522;
        let limit2 = 1698528;
        let invalids = find_invalid_ids(limit1, limit2);
        assert_eq!(invalids, vec![]);
    }
}

