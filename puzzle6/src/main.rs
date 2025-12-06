use std::io::{stdin, BufRead, Read};

#[derive(Eq, PartialEq, Debug)]
enum Op {
    Add,
    Mult,
}

#[derive(Debug)]
enum Word {
    Number(u64),
    Op(Op),
}

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        match value {
            "*" => Self::Op(Op::Mult),
            "+" => Self::Op(Op::Add),
            x => Self::Number(x.parse::<u64>().expect("invalid number")),
        }
    }
}

fn main() {
    let mut reading_line: String = Default::default();
    let _ = stdin().lock().read_to_string(&mut reading_line);

    let lines = reading_line.split("\n");
    let words = lines
        .flat_map(|line| line.split(" "))
        .filter(|word| word.len() != 0)
        .map(|word| word.into());

    let operations: Vec<Op> = words
        .clone()
        .filter_map(|word| {
            if let Word::Op(x) = word {
                Some(x)
            } else {
                None
            }
        })
        .collect();
    let numbers: Vec<u64> = words
        .clone()
        .filter_map(|word| {
            if let Word::Number(number) = word {
                Some(number)
            } else {
                None
            }
        })
        .collect();

    let width = operations.len();
    let height = numbers.len() / width;

    assert_eq!(
        numbers.len() % width,
        0,
        "the inputs should be squared right"
    );
    println!("width: {width}");
    println!("height: {height}");

    let mut grand_total = 0;
    for i in 0..width {
        let mut mult_result = 1;
        let mut add_result = 0;
        for j in 0..height {
            let idx = (j * width) + i;
            let number = numbers[idx];
            mult_result *= number;
            add_result += number;
        }

        match operations[i] {
            Op::Add => grand_total += add_result,
            Op::Mult => grand_total += mult_result,
        }
    }

    println!("{grand_total}");
}
