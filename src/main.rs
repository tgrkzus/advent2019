use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::env;
use itertools::Itertools;

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
            day1::run(&input)
        },
        2 => {
            day2::run(&input)
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

mod day2 {
    use super::*;

    #[derive(Debug)]
    struct Position {
        value: usize
    }

    impl Position {
        pub fn new(value: usize) -> Self {
            Self {
                value
            }
        }

        pub fn mutate(&mut self, new_value: usize) {
            self.value = new_value;
        }

        pub fn value(&self) -> usize {
            self.value
        }
    }

    #[derive(Debug)]
    struct Program {
        positions: Vec<Position>,
        pointer: usize
    }

    impl Program {
        pub fn from_input(input: &Vec<String>) -> Self {
            if input.len() != 1 {
                panic!("input should all be a single line");
            }
            let list = &input[0];
            let positions: Vec<Position> = list.split(",")
                .map(|v| v.parse().unwrap())
                .map(|v| Position::new(v))
                .collect();

            Self {
                positions,
                pointer: 0
            }
        }

        pub fn run_next_op(&mut self) -> bool {
            let op = self.get_op();
            self.pointer += 4;
            return match op {
                Op::Stop => false,
                _ => {
                    let mut_operation = self.get_op_result(op);
                    self.positions[mut_operation.position].mutate(mut_operation.value);
                    true
                },
            }
        }

        fn get_op(&self) -> Op {
            let next_op_value = self.positions[self.pointer].value;
            return match next_op_value {
                99 => Op::Stop,
                1 => {
                    let a1 = self.positions[self.pointer + 1].value;
                    let a2 = self.positions[self.pointer + 2].value;
                    let a3 = self.positions[self.pointer + 3].value;
                    Op::Add(a1, a2, a3)
                },
                2 => {
                    let a1 = self.positions[self.pointer + 1].value;
                    let a2 = self.positions[self.pointer + 2].value;
                    let a3 = self.positions[self.pointer + 3].value;
                    Op::Multiply(a1, a2, a3)
                },
                _ => panic!("we fucked up -- op code is invalid")
            }
        }

        fn get_op_result(&self, op: Op) -> MutatingOpResult {
            return match op {
                Op::Add(a1, a2, r) => {
                    let a1_val = self.positions[a1].value;
                    let a2_val = self.positions[a2].value;
                    MutatingOpResult {
                        position: r,
                        value: a1_val + a2_val
                    }
                },
                Op::Multiply(a1, a2, r) => {
                    let a1_val = self.positions[a1].value;
                    let a2_val = self.positions[a2].value;
                    MutatingOpResult {
                        position: r,
                        value: a1_val * a2_val
                    }
                }
                Op::Stop => panic!("we fucked - stopping has no mutating result")
            }
        }

        pub fn get_value_at(&self, pos: usize) -> usize {
            self.positions[pos].value()
        }

        pub fn set_value_at(&mut self, pos: usize, value: usize) {
            self.positions[pos].mutate(value);
        }
    }

    // Store value at position
    #[derive(Debug)]
    struct MutatingOpResult {
        pub position: usize,
        pub value: usize,
    }

    #[derive(Debug)]
    enum Op {
        Stop,
        Add(usize, usize, usize),
        Multiply(usize, usize, usize)
    }

    pub fn run(input: &Vec<String>) {
        p1(input);
        p2(input);
    }

    fn p1(input: &Vec<String>) {
        let mut program = Program::from_input(input);
        program.set_value_at(1, 12);
        program.set_value_at(2, 2);
        while program.run_next_op() {}

        println!("P1: Position 0: {}", program.get_value_at(0))
    }

    fn p2(input: &Vec<String>) {
        let (n, v) = run_p2_iteratively(input);
        println!("P2: 1, 2: ({}, {}) -> {}", n, v, 100 * n + v);
    }

    fn run_p2_iteratively(input: &Vec<String>) -> (usize, usize) {
        for n in 0..100 {
            for v in 0..100 {
                let mut program = Program::from_input(input);
                program.set_value_at(1, n);
                program.set_value_at(2, v);
                while program.run_next_op() {}

                if program.get_value_at(0) == 19690720 {
                    return (n, v)
                }
            }
        }
        panic!("no valid combination found");
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_one() {
            let input = vec!(String::from("1,0,0,0,99"));
            let mut program = Program::from_input(&input);
            run_program_for_testing(&mut program);

            assert_eq!(program.get_value_at(0), 2);
        }

        #[test]
        fn test_two() {
            let input = vec!(String::from("2,3,0,3,99"));
            let mut program = Program::from_input(&input);
            run_program_for_testing(&mut program);

            assert_eq!(program.get_value_at(3), 6);
        }

        #[test]
        fn test_three() {
            let input = vec!(String::from("2,4,4,5,99,0"));
            let mut program = Program::from_input(&input);
            run_program_for_testing(&mut program);

            assert_eq!(program.get_value_at(5), 9801);
        }

        #[test]
        fn test_four() {
            let input = vec!(String::from("1,1,1,4,99,5,6,0,99"));
            let mut program = Program::from_input(&input);
            run_program_for_testing(&mut program);

            assert_eq!(program.get_value_at(0), 30);
            assert_eq!(program.get_value_at(1), 1);
            assert_eq!(program.get_value_at(2), 1);
            assert_eq!(program.get_value_at(3), 4);
            assert_eq!(program.get_value_at(4), 2);
            assert_eq!(program.get_value_at(5), 5);
            assert_eq!(program.get_value_at(6), 6);
            assert_eq!(program.get_value_at(7), 0);
            assert_eq!(program.get_value_at(8), 99);
        }

        fn run_program_for_testing(program: &mut Program) {
            while program.run_next_op() {}
        }
    }
}

mod day1 {
    use super::*;

    pub fn run(input: &Vec<String>) {
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
