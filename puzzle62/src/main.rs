use std::io::{stdin, Read};

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add,
    Mult,
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Self::Mult),
            '+' => Ok(Self::Add),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut read_buf: String = Default::default();
    let _ = stdin().lock().read_to_string(&mut read_buf);

    let lines = split_to_lines(&read_buf);
    let grand_total = calculate_total(&lines);
    println!("{grand_total}");
}

fn split_to_lines(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input.split("\n").collect();
    lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|x| *x)
        .collect()
}

fn calculate_total(lines: &[&str]) -> u64 {
    let operators = get_operators_from_lines(&lines);

    let number_lines = &lines[0..lines.len() - 1];

    assert!(number_lines.len() != 0, "should have some number");

    assert_eq!(
        operators[0].0, 0,
        "at least the first operator should be aligned monsueir"
    );

    let mut grand_total = 0;
    for i in 0..operators.len() {
        let (number_idx, operator) = &operators[i];
        let until_idx = match &operators.get(i + 1) {
            Some((idx, _)) => idx - 1,
            None => number_lines[0].len() - 1,
        };

        let number_idx = *number_idx;
        let mut final_numbers = vec![];
        for j in number_idx..=until_idx {
            let number = number_at_index(&number_lines, j);
            if number != 0 {
                final_numbers.push(number);
            }
        }

        let operator_result = match operator {
            Op::Add => final_numbers.into_iter().sum(),
            Op::Mult => final_numbers
                .into_iter()
                .reduce(|a, b| a * b)
                .expect("dafuq"),
        };

        grand_total += operator_result;
    }

    grand_total
}

fn number_at_index(lines: &[&str], idx: usize) -> u64 {
    let mut digits: Vec<u64> = vec![];
    for line in lines {
        let char_at_idx = line.chars().collect::<Vec<_>>()[idx];

        let digit = if !char_at_idx.is_whitespace() {
            char_at_idx
                .to_digit(10)
                .expect(format!("bad character sir {}!", char_at_idx.to_string()).as_str())
        } else {
            continue;
        };

        digits.push(digit.into());
    }
    digits.reverse();

    let mut number = 0;
    for (idx, digit) in digits.iter().enumerate() {
        number += digit * (10_u64.pow(idx as u32))
    }
    number
}

fn get_operators_from_lines(items: &[&str]) -> Vec<(usize, Op)> {
    items
        .last()
        .expect("should have at least one line good sir!")
        .chars()
        .enumerate()
        .filter_map(|(idx, ch)| ch.try_into().ok().map(|i| (idx, i)))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{calculate_total, get_operators_from_lines, number_at_index, split_to_lines, Op};

    #[test]
    fn test_split_to_lines() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";
        let res = split_to_lines(input);
        assert_eq!(
            res,
            vec![
                "123 328  51 64 ",
                " 45 64  387 23 ",
                "  6 98  215 314",
                "*   +   *   +  "
            ]
        );
    }

    #[test]
    fn test_calculate_total_with_split() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";
        let lines = split_to_lines(input);
        let result = calculate_total(&lines);
        assert_eq!(result, 3263827);
    }

    #[test]
    fn gathering_operators() {
        let operators = "+ * +";
        let res = get_operators_from_lines(&vec![operators]);
        assert_eq!(res, vec![(0, Op::Add), (2, Op::Mult), (4, Op::Add)]);
    }

    #[test]
    fn number_at_index_parsing_1() {
        let numbers = vec![" 1", "2 3"];
        let res = number_at_index(&numbers, 0);
        assert_eq!(res, 2);
    }

    #[test]
    fn number_at_index_parsing_2() {
        let numbers = vec!["  1", "2 3"];
        let res = number_at_index(&numbers, 2);
        assert_eq!(res, 13);
    }

    #[test]
    fn number_at_index_parsing_3() {
        let numbers = vec![" 11", "203"];
        let res = number_at_index(&numbers, 1);
        assert_eq!(res, 10);
    }

    #[test]
    fn number_at_index_parsing_4() {
        let numbers = vec![" 1 ", "203"];
        let res = number_at_index(&numbers, 1);
        assert_eq!(res, 10);
    }

    #[test]
    fn number_at_index_parsing_5() {
        let numbers = vec!["  2", "213"];
        let res = number_at_index(&numbers, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn number_at_index_parsing_6() {
        let numbers = vec!["  2", "213", "213"];
        let res = number_at_index(&numbers, 1);
        assert_eq!(res, 11);
    }

    #[test]
    fn number_at_index_parsing_7() {
        let numbers = vec!["2  ", "213", "213"];
        let res = number_at_index(&numbers, 0);
        assert_eq!(res, 222);
    }

    #[test]
    fn number_at_index_parsing_8() {
        let numbers = vec![" 2", "213", "213"];
        let res = number_at_index(&numbers, 0);
        assert_eq!(res, 22);
    }

    #[test]
    fn number_at_index_parsing_9() {
        let numbers = vec![" 22", "203", "203"];
        let res = number_at_index(&numbers, 1);
        assert_eq!(res, 200);
    }

    #[test]
    fn number_at_index_parsing_10() {
        let numbers = vec![" 22", "203", "243"];
        let res = number_at_index(&numbers, 1);
        assert_eq!(res, 204);
    }

    #[test]
    fn number_at_index_parsing_11() {
        let numbers = vec!["2  ", "213", "213"];
        let res = number_at_index(&numbers, 2);
        assert_eq!(res, 33);
    }
}
