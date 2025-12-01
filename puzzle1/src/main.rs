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
}

impl Position {
    fn new() -> Self {
        Self { pos: 50 }
    }

    fn rotate_to(&mut self, position: Rotation) {
        match position {
            Rotation::Left(amt) => {
                let new_raw_pos = (self.pos - amt) % (MAX_POS + 1);
                if new_raw_pos < 0 {
                    self.pos = (MAX_POS + 1) + new_raw_pos;
                } else {
                    self.pos = new_raw_pos;
                };
            }
            Rotation::Right(amt) => {
                self.pos = (self.pos + amt) % (MAX_POS + 1);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut position = Position::new();
    let mut password = 0;
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

                if position.pos == 0 {
                    password += 1;
                }
            }
        } else {
            eprintln!("Error reading line");
            exit(1);
        }
    }

    println!("{password}");
    println!("{count}");
}

#[cfg(test)]
mod test {
    use crate::Position;

    #[test]
    fn test_r_rotation_below_99() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(10));
        assert_eq!(position.pos, 60);
    }
    #[test]
    fn test_r_rotation_50() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(50));
        assert_eq!(position.pos, 0);
    }

    #[test]
    fn test_r_rotation_above_99() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Right(60));
        assert_eq!(position.pos, 10);
    }

    #[test]
    fn test_l_rotation_above_99() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(150));
        assert_eq!(position.pos, 0);
    }

    #[test]
    fn test_l_rotation_50() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(50));
        assert_eq!(position.pos, 0);
    }

    #[test]
    fn test_l_rotation_below_99() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(60));
        assert_eq!(position.pos, 90);
    }

    #[test]
    fn test_l_rotation_51() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(51));
        assert_eq!(position.pos, 99);
    }

    #[test]
    fn test_l_rotation_101() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(151));
        assert_eq!(position.pos, 99);
    }

    #[test]
    fn case_1() {
        let mut position = Position::new();
        position.rotate_to(crate::Rotation::Left(68));
        assert_eq!(position.pos, 82);
        position.rotate_to(crate::Rotation::Left(30));
        assert_eq!(position.pos, 52);
        position.rotate_to(crate::Rotation::Right(48));
        assert_eq!(position.pos, 0);
        position.rotate_to(crate::Rotation::Left(5));
        assert_eq!(position.pos, 95);
    }
}
