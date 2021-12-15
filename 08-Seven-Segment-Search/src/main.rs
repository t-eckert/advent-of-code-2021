#![feature(entry_insert)]
use std::{
    collections::{HashMap, HashSet},
    fs,
};

mod entry;

use entry::Entry;

fn main() {
    let filename = "./input.txt";
    let entries = read_entries(filename);
    let unique_count = count_uniques(&entries);

    println!("Count of 1s, 4s, 7s, and 8s: {}", unique_count);

    let output_sum = sum_outputs(&entries);

    println!("Sum of interpreted outputs: {}", output_sum);
}

// Solves part 1
fn count_uniques(entries: &Vec<Entry>) -> i32 {
    let unique_lengths = vec![2, 4, 3, 7];

    let mut uniques = 0;
    for entry in entries.into_iter() {
        for output in &entry.output {
            if unique_lengths.contains(&output.len()) {
                uniques += 1;
            }
        }
    }

    uniques
}

// Solves part 2
fn sum_outputs(entries: &Vec<Entry>) -> i32 {
    entries
        .into_iter()
        .fold(0, |sum, entry| sum + interpret_entry(entry))
}

fn interpret_entry(entry: &Entry) -> i32 {
    let wiring_map = translate_wiring(&entry.wirings);

    entry
        .output
        .iter()
        .rev()
        .enumerate()
        .map(|(index, elem)| {
            wiring_map
                .get(&alphabetize(&elem.chars().collect()))
                .unwrap()
                * 10_i32.pow(index.try_into().unwrap())
        })
        .sum()
}

fn translate_wiring(wiring: &Vec<String>) -> HashMap<String, i32> {
    let by_length = group_by_length(&wiring);

    // The wirings for 1, 4, 7, and 8 are known by the unique number of segments they light up.
    let one_wiring = by_length.get(&2).unwrap().last().unwrap();
    let four_wiring = by_length.get(&4).unwrap().last().unwrap();
    let seven_wiring = by_length.get(&3).unwrap().last().unwrap();
    let eight_wiring = by_length.get(&7).unwrap().last().unwrap();

    // 3's wiring must light up five segments and include both segements in 1.
    let three_wiring = by_length
        .get(&5)
        .unwrap()
        .iter()
        .filter(|wiring| wiring.is_superset(one_wiring))
        .last()
        .unwrap();

    let lower_cross_bars: HashSet<char> = three_wiring
        .difference(seven_wiring)
        .map(|ref_char| ref_char.to_owned())
        .collect();

    // 5's wiring must light up five segments, have all but one segment in 4's, and not both values in 1's.
    let five_wiring = by_length
        .get(&5)
        .unwrap()
        .iter()
        .filter(|wiring| {
            wiring
                .intersection(four_wiring)
                .collect::<HashSet<&char>>()
                .len()
                == 3
                && !wiring.is_superset(one_wiring)
        })
        .last()
        .unwrap();

    // 2's wiring must light up five segments, and not be 3's or 5's wiring.
    let two_wiring = by_length
        .get(&5)
        .unwrap()
        .iter()
        .filter(|wiring| wiring != &three_wiring && wiring != &five_wiring)
        .last()
        .unwrap();

    // 0's wiring must light up six segments and only 1 of the segments in the lower cross bars.
    let zero_wiring = by_length
        .get(&6)
        .unwrap()
        .iter()
        .filter(|wiring| !wiring.is_superset(&lower_cross_bars))
        .last()
        .unwrap();

    // 6's wiring must light up six segments, not be 0's wiring, and only 1 of the segments in 1's wiring.
    let six_wiring = by_length
        .get(&6)
        .unwrap()
        .iter()
        .filter(|wiring| {
            one_wiring
                .intersection(&wiring)
                .into_iter()
                .map(|ref_char| ref_char.clone())
                .collect::<Vec<char>>()
                .len()
                == 1
                && wiring != &zero_wiring
        })
        .last()
        .unwrap();

    // 9's wiring must light up six segments, not be 0, and not be 6.
    let nine_wiring = by_length
        .get(&6)
        .unwrap()
        .iter()
        .filter(|wiring| wiring != &zero_wiring && wiring != &six_wiring)
        .last()
        .unwrap();

    let mut wiring_map: HashMap<String, i32> = HashMap::new();
    wiring_map.insert(alphabetize(zero_wiring), 0);
    wiring_map.insert(alphabetize(one_wiring), 1);
    wiring_map.insert(alphabetize(two_wiring), 2);
    wiring_map.insert(alphabetize(three_wiring), 3);
    wiring_map.insert(alphabetize(four_wiring), 4);
    wiring_map.insert(alphabetize(five_wiring), 5);
    wiring_map.insert(alphabetize(six_wiring), 6);
    wiring_map.insert(alphabetize(seven_wiring), 7);
    wiring_map.insert(alphabetize(eight_wiring), 8);
    wiring_map.insert(alphabetize(nine_wiring), 9);

    wiring_map
}

fn group_by_length(wirings: &Vec<String>) -> HashMap<usize, Vec<HashSet<char>>> {
    let mut by_length: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
    for wiring in wirings {
        by_length
            .entry(wiring.len())
            .and_modify(|existing: &mut Vec<HashSet<char>>| existing.push(wiring.chars().collect()))
            .or_insert(vec![wiring.chars().collect()]);
    }

    by_length
}

fn alphabetize(string: &HashSet<char>) -> String {
    let mut word = string
        .iter()
        .map(|ref_char| ref_char.clone())
        .collect::<Vec<char>>();
    word.sort_by(|a, b| a.cmp(b));

    String::from_iter(word)
}

fn read_entries(path: &str) -> Vec<Entry> {
    let content = fs::read_to_string(path).expect("Could not read file!");
    content
        .lines()
        .map(|entry| Entry::from_str(entry))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpret_entry() {
        let input =
            "bgcfda gecbda abdgf aedfbg eda efcbd ae agfe bdefagc fbeda | ae egdafb ea fcdeb";

        let entry = Entry::from_str(input);

        let actual = interpret_entry(&entry);

        assert_eq!(1912, actual);
    }

    #[test]
    fn test_translate_wiring() {
        let input: Vec<String> = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
            .split(" ")
            .map(|s| s.to_owned())
            .collect();
        let mut expected: HashMap<String, i32> = HashMap::new();
        expected.insert("abcdeg".to_owned(), 0);
        expected.insert("ab".to_owned(), 1);
        expected.insert("acdfg".to_owned(), 2);
        expected.insert("abcdf".to_owned(), 3);
        expected.insert("abef".to_owned(), 4);
        expected.insert("bcdef".to_owned(), 5);
        expected.insert("abd".to_owned(), 7);
        expected.insert("bcdefg".to_owned(), 6);
        expected.insert("abcdefg".to_owned(), 8);
        expected.insert("abcdef".to_owned(), 9);

        let actual = translate_wiring(&input);

        assert_eq!(expected, actual);
    }
}
