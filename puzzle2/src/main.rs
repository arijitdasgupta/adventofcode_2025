use std::fs;

fn is_invalid(i: u64) -> bool {
    let i = i.to_string();
    let half_len = i.len() / 2;

    if i.len() == 2 && i.parse::<u64>().unwrap() % 11 == 0 {
        true
    } else if i.len() % 2 == 0 && &i[..half_len] == &i[half_len..] {
        true
    } else {
        false
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
    use crate::{find_invalid_ids, is_invalid};

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
    fn test_invalid_false_2() {
        let n: u64 = 112211;
        assert_eq!(is_invalid(n), false);
    }

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
        assert_eq!(invalids, vec![99]);
    }

    #[test]
    fn test_finder_3() {
        let limit1 = 998;
        let limit2 = 1012;
        let invalids = find_invalid_ids(limit1, limit2);
        assert_eq!(invalids, vec![1010]);
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
