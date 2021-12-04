use std::fs;

fn main() {
    let filename = "./src/input.txt";
    let content = fs::read_to_string(filename).expect("Could not read file!");
    let diagnostic: Vec<&str> = content.lines().collect();

    let binary_tensor: Vec<Vec<bool>> = diagnostic
        .into_iter()
        .map(|binary| parse_to_vec(binary))
        .collect();

    let length = &binary_tensor.len();
    let col_count = count_columns(binary_tensor);
    println!("{:?}", col_count);

    let g = gamma(&col_count, &length);
    let e = epsilon(&col_count, &length);

    println!("Gamma: {}", g);
    println!("Epsilon: {}", e);
    println!("Power consumption: {}", g * e);
}

fn parse_to_vec(binary: &str) -> Vec<bool> {
    binary
        .chars()
        .map(|c| {
            if c == '0' {
                return false;
            } else if c == '1' {
                return true;
            } else {
                panic!("That's not binary! No fair!");
            }
        })
        .collect()
}

fn count_columns(diagnostic: Vec<Vec<bool>>) -> Vec<usize> {
    // We assume no jaggedness.
    let n_cols = diagnostic[0].len();

    let mut count = vec![0; n_cols];
    for i in 0..n_cols {
        for j in 0..diagnostic.len() {
            count[i] = count[i] + usize::from(diagnostic[j][i]);
        }
    }

    count
}

fn gamma(col_count: &Vec<usize>, length: &usize) -> usize {
    let mut num = 0;
    for (index, col) in col_count.iter().enumerate() {
        let place = col_count.len() - index - 1;
        if *col > length / 2 {
            num = num + 2usize.pow(place.try_into().unwrap());
        }
    }

    num
}

fn epsilon(col_count: &Vec<usize>, length: &usize) -> usize {
    let mut num = 0;
    for (index, col) in col_count.iter().enumerate() {
        let place = col_count.len() - index - 1;
        if *col < length / 2 {
            num = num + 2usize.pow(place.try_into().unwrap());
        }
    }

    num
}
