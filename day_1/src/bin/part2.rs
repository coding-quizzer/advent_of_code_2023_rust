use std::{
    collections::HashMap,
    str::{Chars, Lines},
};
const NUMBER_ARRAY: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug, Default)]
struct NumberFound {
    number: &'static str,
    index: usize,
}

fn main() {
    let input_vec = include_str!("./input.txt").lines();
    let word_to_number_hashap = create_number_word_to_numeral_map(NUMBER_ARRAY);
    println!(
        "Part 2 sum: {}",
        part2(input_vec, NUMBER_ARRAY, word_to_number_hashap)
    );
}
fn part2(
    input: Lines,
    number_word_list: [&'static str; 10],
    hashmap: HashMap<&'static str, u32>,
) -> u32 {
    let mut number_list = Vec::new();
    for line in input {
        let first_digit = get_first_spelled_out_number(line, number_word_list, &hashmap)
            .expect("There must be at least one number in the input");
        let last_digit = get_last_spelled_out_number(line, number_word_list, &hashmap)
            .expect("There must be at least one number in the input");
        number_list.push(first_digit * 10 + last_digit);
    }
    println!("Number List: {number_list:#?}");
    number_list.iter().sum::<u32>()
}

fn create_number_word_to_numeral_map(
    number_word_list: [&'static str; 10],
) -> HashMap<&'static str, u32> {
    let number_numerals = 0..=9;
    number_word_list
        .iter()
        .cloned()
        .zip(number_numerals)
        .collect()
}

fn get_first_numeral_number(input_str: &str) -> (Option<usize>, Option<u32>) {
    let char_list = input_str.chars();
    for (index, char) in char_list.enumerate() {
        if let Some(number) = char.to_digit(10) {
            return (Some(index), Some(number));
        }
    }
    (None, None)
}

fn get_last_numeral_number(input_str: &str) -> (Option<usize>, Option<u32>) {
    let str_len = input_str.len();
    let char_list = input_str.chars();

    for (index, char) in char_list.rev().enumerate() {
        if let Some(number) = char.to_digit(10) {
            return (Some((str_len - 1) - index), Some(number));
        }
    }
    (None, None)
}

fn get_first_spelled_out_number(
    input: &str,
    number_word_list: [&str; 10],
    hashmap: &HashMap<&'static str, u32>,
) -> Option<u32> {
    // let current_number = NumberFound {
    //     number: "",
    //     index: usize::MAX,
    // };

    let (mut current_index, mut current_number) = get_first_numeral_number(input);
    for number in number_word_list {
        if let Some(index) = input.find(number) {
            if current_index.is_none() || index <= current_index.unwrap() {
                current_index = Some(index);
                current_number = Some(*(hashmap.get(number).unwrap()));
            }

            if index == 0 {
                break;
            };
        };
    }

    current_number
}
fn get_last_spelled_out_number(
    input: &str,
    number_word_list: [&str; 10],
    hashmap: &HashMap<&'static str, u32>,
) -> Option<u32> {
    // let current_number = NumberFound {
    //     number: "",
    //     index: usize::MAX,
    // };

    let (mut current_index, mut current_number) = get_last_numeral_number(input);
    // println!("Current index: {:?}", current_index);
    for number in number_word_list {
        if let Some(index) = input.rfind(number) {
            if current_index.is_none() || index > current_index.unwrap() {
                current_index = Some(index);
                current_number = Some(*hashmap.get(number).unwrap());
            }
        };
    }

    current_number
}
