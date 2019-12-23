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
