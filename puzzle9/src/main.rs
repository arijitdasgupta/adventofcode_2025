use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn compute_largest_rect(input: &str) -> i64 {
    let points: Vec<Point> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            Point {
                x: parts[0].trim().parse().expect("Invalid x"),
                y: parts[1].trim().parse().expect("Invalid y"),
            }
        })
        .collect();

    let mut max_area: i64 = 1;
    for i in 0..points.len() {
        for j in 0..points.len() {
            let area = calculate_area(points[i], points[j]);
            if max_area < area {
                max_area = area;
            }
        }
    }
    max_area
}

fn calculate_area(p1: Point, p2: Point) -> i64 {
    let width = (p1.x - p2.x).abs() + 1;
    let height = (p1.y - p2.y).abs() + 1;
    width * height
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let max_area = compute_largest_rect(&input);
    println!("{max_area}");
}

#[cfg(test)]
mod test {
    use crate::{Point, calculate_area, compute_largest_rect};

    #[test]
    fn test_input_1() {
        let input = include_str!("../test_input1.txt");
        let max_area = compute_largest_rect(input);
        assert_eq!(max_area, 50);
    }

    #[test]
    fn test_calculate_area_basic() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert_eq!(calculate_area(p1, p2), 20);
    }

    #[test]
    fn test_calculate_area_same_point() {
        let p1 = Point { x: 5, y: 5 };
        let p2 = Point { x: 5, y: 5 };
        assert_eq!(calculate_area(p1, p2), 1);
    }

    #[test]
    fn test_calculate_area_one() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 5, y: 0 };
        assert_eq!(calculate_area(p1, p2), 6);
    }
}
