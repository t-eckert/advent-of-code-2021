use std::fs;

fn main() {
    let input = read_input("input.txt");

    let fuel_constant = min_fuel_constant_burn(&input);
    println!(
        "Minimum fuel required with constant burn: {}",
        fuel_constant
    );

    let fuel_increasing = min_fuel_increasing_burn(&input);
    println!(
        "Minimum fuel required with increasing burn: {}",
        fuel_increasing
    );
}

fn min_fuel_constant_burn(crabs: &Vec<i32>) -> i32 {
    let mut crabs = crabs.clone();
    crabs.sort();

    let median = crabs.get(&crabs.len() / 2).unwrap();

    crabs.iter().map(|crab| (crab - median).abs()).sum()
}

fn min_fuel_increasing_burn(crabs: &Vec<i32>) -> i32 {
    let mut crabs = crabs.clone();
    crabs.sort();

    let targets = 0..=*crabs.last().unwrap();

    targets
        .map(|target| {
            crabs
                .iter()
                .map(|crab| (0..=(crab - target).abs()).sum::<i32>())
                .sum()
        })
        .min()
        .unwrap()
}

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .unwrap()
        .split(",")
        .map(|elem| elem.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_fuel_constant_burn() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let expected = 37;

        let actual = min_fuel_constant_burn(&crabs);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_min_fuel_increasing_burn() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let expected = 168;

        let actual = min_fuel_increasing_burn(&crabs);

        assert_eq!(expected, actual);
    }
}
