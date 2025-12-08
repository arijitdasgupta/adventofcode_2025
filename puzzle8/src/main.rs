use std::{
    collections::BTreeMap,
    io::{self, Read},
};

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let points: Vec<Point> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            Point {
                x: parts[0].trim().parse().expect("Invalid x"),
                y: parts[1].trim().parse().expect("Invalid y"),
                z: parts[2].trim().parse().expect("Invalid z"),
            }
        })
        .collect();

    println!("Loaded {} points", points.len());
    if !points.is_empty() {
        println!("First point: {:?}", points[0]);
    }

    let mut distances = BTreeMap::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i != j && i < j {
                distances.insert((i, j), points[i].distance(&points[j]));
            }
        }
    }

    println!("{}", distances.iter().count());

    let mut distances = distances
        .into_iter()
        .collect::<Vec<((usize, usize), f64)>>();

    distances.sort_by(|(_, distance1), (_, distance2)| distance1.total_cmp(distance2));
    println!("{:?} {:?}", distances[0], distances[1]);
}
