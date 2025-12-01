use std::{
    io::{self, BufRead},
    process::exit,
};

const MAX_POS: i64 = 99;

enum Rotation {
    Left(i64),
    Right(i64),
}

struct Position {
    pos: i64,
    zero_encounter: i64,
}

impl Position {
    fn new() -> Self {
        Self {
            pos: 50,
            zero_encounter: 0,
        }
    }

    fn rotate_to(&mut self, rotation: Rotation) {
        let new_raw_pos = match rotation {
            Rotation::Left(amt) => (self.pos - amt) % (MAX_POS + 1),
            Rotation::Right(amt) => (self.pos + amt) % (MAX_POS + 1),
        };

        let zero_encounters = match rotation {
            Rotation::Left(amt) => {
                if self.pos > amt {
                    0
                } else if self.pos == amt {
                    1
                } else {
                    (amt - self.pos) / (MAX_POS + 1) + (self.pos != 0) as i64
                }
            }
            Rotation::Right(amt) => (self.pos + amt) / (MAX_POS + 1),
        };

        self.zero_encounter += zero_encounters;

        if new_raw_pos < 0 {
            self.pos = (MAX_POS + 1) + new_raw_pos;
        } else {
            self.pos = new_raw_pos;
        };
    }
}

fn main() {
    let stdin = io::stdin();

    let mut position = Position::new();
    let mut count = 0;

    for line in stdin.lock().lines() {
        count += 1;
        if let Ok(content) = line {
            let mut chars = content.chars();

            if let Some(first) = chars.next() {
                let direction = first;
                let num_str: String = chars.collect();
                let num = num_str.parse::<i64>().unwrap();

                let rotation = match direction {
                    'L' => Rotation::Left(num),
                    'R' => Rotation::Right(num),
                    _ => {
                        eprintln!("bad input");
                        exit(1);
                    }
                };
                position.rotate_to(rotation);
            }
        } else {
            eprintln!("Error reading line");
            exit(1);
        }
    }

    println!("Answer: {}", position.zero_encounter);
    println!("Line count: {count}");
}

#[cfg(test)]
mod test {
    use crate::Position;

    #[test]
    fn test_r_rotation_below_99() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(10));
        assert_eq!(position.pos, 60);
        assert_eq!(position.zero_encounter, 0);
    }
    #[test]
    fn test_r_rotation_50() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(50));
        assert_eq!(position.pos, 0);
        assert_eq!(position.zero_encounter, 1);
    }

    #[test]
    fn test_r_rotation_above_99_1() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(60));
        assert_eq!(position.pos, 10);
        assert_eq!(position.zero_encounter, 1);
    }

    #[test]
    fn test_r_rotation_above_99_2() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(100));
        assert_eq!(position.pos, 50);
        assert_eq!(position.zero_encounter, 1);
    }

    #[test]
    fn test_r_rotation_above_99_3() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(150));
        assert_eq!(position.pos, 0);
        assert_eq!(position.zero_encounter, 2);
    }

    #[test]
    fn test_l_rotation_above_99() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(150));
        assert_eq!(position.zero_encounter, 2);
        assert_eq!(position.pos, 0);
    }

    #[test]
    fn test_l_rotation_50() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(50));
        assert_eq!(position.pos, 0);
        assert_eq!(position.zero_encounter, 1);
    }

    #[test]
    fn test_l_rotation_below_99() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(60));
        assert_eq!(position.pos, 90);
        assert_eq!(position.zero_encounter, 1);
    }

    #[test]
    fn test_l_rotation_51() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(51));
        assert_eq!(position.pos, 99);
        assert_eq!(position.zero_encounter, 1);
    }

    #[test]
    fn test_l_rotation_151() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(151));
        assert_eq!(position.pos, 99);
        assert_eq!(position.zero_encounter, 2);
    }

    #[test]
    fn case_1() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(68));
        assert_eq!(position.pos, 82);
        assert_eq!(position.zero_encounter, 1);
        position.rotate_to(crate::Rotation::Left(30));
        assert_eq!(position.pos, 52);
        position.rotate_to(crate::Rotation::Right(48));
        assert_eq!(position.pos, 0);
        assert_eq!(position.zero_encounter, 2);
        position.rotate_to(crate::Rotation::Left(5));
        assert_eq!(position.pos, 95);
        assert_eq!(position.zero_encounter, 2);
        position.rotate_to(crate::Rotation::Right(60));
        assert_eq!(position.pos, 55);
        assert_eq!(position.zero_encounter, 3);
        position.rotate_to(crate::Rotation::Left(55));
        assert_eq!(position.pos, 0);
        assert_eq!(position.zero_encounter, 4);
    }
}
