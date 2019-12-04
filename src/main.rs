use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let problem: u32 = args[1].parse().unwrap();
            let file: String = args[1].parse().unwrap();
            run_problem(problem, file)
        }
        _ => {
            println!("Args wrong... ./advent <problem 1-n>")
        }
    }
}

fn run_problem(problem: u32, file: String) {
    println!("Running problem {} with file {}", problem, file);
    let input = get_input(&file);
    match problem {
        1 => {
            day1::day_one(&input)
        },
        _ => {
            println!("Problem {} not found.", problem)
        }
    }
}

fn get_input(file: &str) -> Vec<String> {
    let f = File::open(format!("inputs/{}", file)).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect()
}

mod day1 {
    use super::*;

    pub fn day_one(input: &Vec<String>) {
        p1(input);
        p2(input);
    }

    fn p1(input: &Vec<String>) {
        let result: i32 = input.iter()
            .map(|v| v.parse().unwrap())
            .map(self::weight_to_fuel)
            .sum();
        println!("P1 Answer: {}", result)
    }

    fn p2(input: &Vec<String>) {
        let result : i32 = input.iter()
            .map(|v| v.parse().unwrap())
            .map(self::weight_to_fuel_recursive)
            .sum();
        println!("P2 Answer: {}", result)
    }

    fn weight_to_fuel(value: i32) -> i32 {
        return (value / 3) - 2;
    }

    fn weight_to_fuel_recursive(value: i32) -> i32 {
        let fuel_required = weight_to_fuel(value);
        if fuel_required < 0 {
            return 0;
        }
        return fuel_required + weight_to_fuel_recursive(fuel_required);

    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_weight_to_fuel() {
            assert_eq!(weight_to_fuel(12), 2);
            assert_eq!(weight_to_fuel(14), 2);
            assert_eq!(weight_to_fuel(1969), 654);
            assert_eq!(weight_to_fuel(100756), 33583);

            assert_ne!(weight_to_fuel(12412412), 412);
        }

        #[test]
        fn test_weight_to_fuel_recursive() {
            assert_eq!(weight_to_fuel_recursive(12), 2);
            assert_eq!(weight_to_fuel_recursive(1969), 966);
            assert_eq!(weight_to_fuel_recursive(100756), 50346);

            assert_ne!(weight_to_fuel_recursive(100756), 33583);
        }
    }
}
