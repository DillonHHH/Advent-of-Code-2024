mod custom_utils;

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in stdin.lock().lines() {
        let unwrapped_line = line.unwrap();
        let split_string: Vec<_> = unwrapped_line.split_whitespace().collect();
        if split_string[0] == "\\0" {
            break;
        }

        left_list.push(split_string[0].parse().unwrap());
        right_list.push(split_string[1].parse().unwrap());
    }

    let left_list_len = left_list.len();
    custom_utils::quick_sort(&mut left_list, 0, left_list_len - 1);

    let right_list_len = right_list.len();
    custom_utils::quick_sort(&mut right_list, 0, right_list_len - 1);

    let mut subtraction_stack: Vec<i32> = Vec::new();

    for num in 0..left_list.len() {
        subtraction_stack.push((left_list[num] - right_list[num]).abs());
    }

    let mut total = 0;
    while !subtraction_stack.is_empty() {
        total += subtraction_stack.pop().unwrap();
    }

    println!("total: {}", total);
    println!(
        "similarity: {}",
        calculate_similarity_scores(left_list, right_list)
    );
}

fn calculate_similarity_scores(sorted_left_list: Vec<i32>, sorted_right_list: Vec<i32>) -> i32 {
    let mut similarity_score = 0;

    let mut left_index = 0;
    let mut right_index = 0;
    //for left_index in 0..sorted_left_list.len() {
    loop {
        let current_num = sorted_left_list[left_index];
        let mut left_num_count = 0;
        let mut right_num_count = 0;

        while sorted_left_list[left_index] == current_num {
            left_num_count += 1;
            left_index += 1;
            if left_index >= sorted_left_list.len() {
                break;
            }
        }

        while sorted_right_list[right_index] <= current_num {
            if sorted_right_list[right_index] == current_num {
                right_num_count += 1;
            }

            right_index += 1;
            if right_index >= sorted_right_list.len() {
                break;
            }
        }

        similarity_score += (current_num * left_num_count) * right_num_count;

        if left_index >= sorted_left_list.len() || right_index >= sorted_right_list.len() {
            break;
        }
    }

    return similarity_score;
}
