
fn main() {
    let measurements = read_from_csv("./src/input.csv").unwrap();   

    let depth_increase = sweep_sonar(&measurements);
    println!("Depth increase\t{}", depth_increase);

    let windowed_depth_increase = sweep_sonar(&window_sum(&measurements, 3));
    println!("Windowed depth increase\t{}", windowed_depth_increase);
}

fn read_from_csv(filename: &str) -> Result<Vec<u32>, &'static str> {
    let mut reader = csv::Reader::from_path(filename).expect("cannot access file");

    let raw_numbers = reader.deserialize::<u32>();

    let mut input = vec![];
    for raw_number in raw_numbers {
        match raw_number {
            Ok(number) => input.push(number),
            _ => return Err("could not parse to u32"),
        }
    }

    return Ok(input)
}

fn sweep_sonar(measurements: &Vec<u32>) -> u32 {
    measurements
        .windows(2)
        .fold(0, |count, window| count + (window[0] < window[1]) as u32)
}

fn window_sum(measurements: &Vec<u32>, window_size: usize) -> Vec<u32> {
    measurements
        .windows(window_size)
        .map(| window| window.iter().sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sonar_sweep_part_1_0() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;

        let actual = sweep_sonar(&measurements);

        assert_eq!(expected, actual);
    }


    #[test]
    fn test_sonar_sweep_part_1_1() {
        let measurements = vec![0, 1, 0, 3];
        let expected = 2;

        let actual = sweep_sonar(&measurements);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sonar_sweep_part_2_0() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;

        let actual = sweep_sonar(&window_sum(&measurements, 3));

        assert_eq!(expected, actual);
    }
}