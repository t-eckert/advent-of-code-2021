#![feature(entry_insert)]
#![allow(dead_code)]
use std::{collections::HashMap, fs};

fn main() {
    let n_steps: u32 = 256;
    let initial_fish = read_input("./src/input.txt");

    let final_fish = simulate_indexed(initial_fish, n_steps);

    println!("\nFish count: {}", final_fish);
}

fn read_input(path: &str) -> Vec<u8> {
    let contents = fs::read_to_string(path).expect("Could not read input.");
    contents
        .split(",")
        .map(|value| value.parse().expect("Could not parse value"))
        .collect()
}

fn simulate_indexed(initial_fish: Vec<u8>, n_steps: u32) -> usize {
    let mut fish = map_fish(initial_fish);
    let max_age = 8;

    for day in 0..n_steps {
        fish = step_indexed(&fish, max_age);
        println!("Day {}", day + 1);
    }

    let mut fish_count = 0;
    for age in 0..=max_age {
        fish_count += fish.get(&age).expect("There are missing fish at some age.");
    }

    fish_count
}

fn map_fish(initial_fish: Vec<u8>) -> HashMap<u8, usize> {
    let mut fish = HashMap::new();
    for f in initial_fish {
        let count = fish.entry(f).or_insert(0);
        *count += 1;
    }
    fish
}

fn step_indexed(fish: &HashMap<u8, usize>, max_age: u8) -> HashMap<u8, usize> {
    let mut next_fish: HashMap<u8, usize> = HashMap::new();

    let n_births = fish.get(&0).unwrap_or(&0);
    for age in 0..=max_age {
        next_fish
            .entry(age)
            .insert(*fish.get(&(age + 1)).unwrap_or(&0));
    }

    let reset_fish = next_fish.entry(6).or_insert(0);
    *reset_fish += n_births;

    let newborns = next_fish.entry(8).or_insert(0);
    *newborns = *n_births;

    next_fish
}

// === Less efficient solution written for part one ===
fn simulate_inline(initial_fish: Vec<u8>, n_steps: u32) -> usize {
    let mut fish = initial_fish.clone();

    for day in 0..n_steps {
        fish = step_inline(fish);
        println!("Day {}", day);
    }

    fish.len()
}

fn step_inline(mut fish: Vec<u8>) -> Vec<u8> {
    let mut n_new: usize = 0;

    fish = fish
        .iter()
        .map(|f| {
            if f == &0 {
                // A new fish is born
                n_new += 1;

                // Fish resets
                return 6;
            }

            *f - 1
        })
        .collect::<Vec<u8>>();
    fish.extend(vec![8; n_new]);

    fish
}
