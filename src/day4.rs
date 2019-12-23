use super::*;
use std::collections::{HashSet, HashMap};

pub fn run(input: &Vec<String>) {
    p1(input);
    p2(input);
}

fn p1(input: &Vec<String>) {
    if input.len() != 1 {
        panic!("bad input len");
    }
    let (start, end): (i32, i32) = {
        let mut inp = input[0].split("-");
        (inp.next().unwrap().parse().unwrap(), inp.next().unwrap().parse().unwrap())
    };
    let mut sum = 0;
    for x in start..=end {
        let x_str = x.to_string();
        let mut valid = true;
        let mut map = HashMap::new();
        let mut last = 0;
        for c in x_str.chars() {
            *map.entry(c).or_insert(0) += 1;
            if last > c.to_digit(10).unwrap() {
                valid = false;
                break;
            }
            last = c.to_digit(10).unwrap();
        }
        valid &= map
            .values()
            .any(|c| *c > 1);

        if valid {
            sum += 1;
        }
    }
    println!("P1: {}", sum);
}

fn p2(input: &Vec<String>) {
    if input.len() != 1 {
        panic!("bad input len");
    }
    let (start, end): (i32, i32) = {
        let mut inp = input[0].split("-");
        (inp.next().unwrap().parse().unwrap(), inp.next().unwrap().parse().unwrap())
    };
    let mut sum = 0;
    for x in start..=end {
        let x_str = x.to_string();
        let mut valid = true;
        let mut map = HashMap::new();
        let mut last = 0;
        for c in x_str.chars() {
            *map.entry(c).or_insert(0) += 1;
            if last > c.to_digit(10).unwrap() {
                valid = false;
                break;
            }
            last = c.to_digit(10).unwrap();
        }
        valid &= map
            .values()
            .any(|c| *c == 2);

        if valid {
            sum += 1;
        }
    }
    println!("P2: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
    }
}
