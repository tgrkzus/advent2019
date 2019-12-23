use super::*;
use std::collections::{HashSet, HashMap};

pub fn run(input: &Vec<String>) {
    p1(input);
    p2(input);
}

fn p1(input: &Vec<String>) {
    if input.len() != 2 {
        panic!("Bad len!");
    }
    let w1 = WireState::from_string(&input[0]);
    let w2 = WireState::from_string(&input[1]);
    let mut set = HashMap::new();
    w1.points.iter().for_each(|(x, y, c)| {
        set.insert((x, y), c);
    });
    let optimal = w2.points
        .iter()
        .filter(|(x, y, _)| set.contains_key(&(x, y)))
        .map(|(x, y, _)| x.abs() + y.abs())
        .min().unwrap();
    println!("P1: {}", optimal);
}

fn p2(input: &Vec<String>) {
    if input.len() != 2 {
        panic!("Bad len!");
    }
    let w1 = WireState::from_string(&input[0]);
    let w2 = WireState::from_string(&input[1]);
    let mut map = HashMap::new();
    w1.points.iter().for_each(|(x, y, c)| {
        if !map.contains_key(&(x, y)) {
            map.insert((x, y), c);
        }
    });
    let optimal = w2.points
        .iter()
        .filter(|(x, y, _)| map.contains_key(&(x, y)))
        .map(|(x, y, c)| {
            *map.get(&(x, y)).unwrap() + c
        })
        .min().unwrap();
    println!("P2: {}", optimal);
}

pub struct WireState {
    pub points: Vec<(i32, i32, u32)>
}

impl WireState {
    pub fn from_string(string: &str) -> Self {
        let mut current = (0, 0, 0);
        let mut points = Vec::new();
        for token in string.split(",") {
            let dir = match &token[0..1] {
                "L" => Dir::L,
                "R" => Dir::R,
                "U" => Dir::U,
                "D" => Dir::D,
                _ => panic!("bad dir"),
            };
            let steps = token[1..].parse().unwrap();
            let (new_current, mut visited) = Self::walk_path(current, dir, steps);
            current = new_current;
            points.append(&mut visited);
        }
        Self {
            points
        }
    }

    fn walk_path(mut current: (i32, i32, u32), path: Dir, mut steps: u32) -> ((i32, i32, u32), Vec<(i32, i32, u32)>) {
        let mut visited = Vec::new();
        while steps != 0 {
            let next = match path {
                Dir::L => (-1, 0),
                Dir::R => (1, 0),
                Dir::U => (0, -1),
                Dir::D => (0, 1),
            };
            current = (current.0 + next.0, current.1 + next.1, current.2 + 1);
            visited.push(current.clone());
            steps -= 1;
        }
        return (current, visited)
    }
}

#[derive(Debug)]
enum Dir {
    L, R, U, D,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let wires = vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")];
        p1(&wires); // 159
    }
}
