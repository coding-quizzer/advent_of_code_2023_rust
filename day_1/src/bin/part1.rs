use std::str::{Chars, Lines};

fn main() {
    let input_vec = include_str!("./input.txt").lines();
    println!("Part 1 sum: {}", part1(input_vec));
}

fn part1(input: Lines) -> u32 {
    let mut number_list = Vec::new();
    for lines in input {
        let char_list = lines.chars();
        let first_digit = get_first_number(char_list.clone())
            .expect("There must be at least one number in the input");
        let last_digit = get_last_number(char_list.clone())
            .expect("There must be at least one number in the input");
        number_list.push(first_digit * 10 + last_digit);
    }
    println!("Number List: {number_list:#?}");
    number_list.iter().sum::<u32>()
}

fn get_first_number(input_list: Chars) -> Option<u32> {
    for char in input_list {
        if let Some(number) = char.to_digit(10) {
            return Some(number);
        }
    }
    None
}

fn get_last_number(input_list: Chars) -> Option<u32> {
    for char in input_list.rev() {
        if let Some(number) = char.to_digit(10) {
            return Some(number);
        }
    }
    None
}
