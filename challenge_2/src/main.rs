use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();

    let mut safety_count = 0;

    let mut lines: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        let unwrapped_line = line.unwrap();
        lines.push(unwrapped_line);
        let split_string: Vec<&str> = lines.last().unwrap().split_whitespace().collect();

        if split_string[0] == "\\0" {
            lines.pop();
            break;
        }
    }

    for line in lines {
        let split_string: Vec<&str> = line.split_whitespace().collect();
        if split_string.len() <= 1 {
            safety_count += 1;
            continue;
        }

        if is_report_safe(&split_string, false) {
            safety_count += 1
        } else {
            println!("unsafe report:\n{:?}", split_string);
        }
    }

    println!("safety count: {}", safety_count);
}

fn is_report_safe(inputs: &Vec<&str>, problem_dampener_used: bool) -> bool {
    let mut increasing = false; // false if decreasing

    if inputs[0].parse::<i32>().unwrap() < inputs[1].parse::<i32>().unwrap() {
        increasing = true;
    }

    let inputs_slice: Vec<_> = inputs[..inputs.len() - 1].to_vec();

    let mut issue_found = false;

    for (index, num) in inputs_slice.iter().enumerate() {
        let num = num.parse::<i32>().unwrap();
        let next_num = inputs[index + 1].parse::<i32>().unwrap();

        if increasing && num > next_num
            || !increasing && num < next_num
            || num - next_num == 0
            || (num - next_num).abs() > 3
        {
            if problem_dampener_used {
                return false;
            } else {
                let mut dampened_inputs = inputs.clone();
                dampened_inputs.remove(index);
                let safe_result = is_report_safe(&dampened_inputs, true);
                if safe_result {
                    return true;
                } else {
                    issue_found = true;
                }
            }
        }
    }

    if issue_found {
        // try removing first and last items
        let mut dampened_inputs = inputs.clone();
        dampened_inputs.remove(0);
        if is_report_safe(&dampened_inputs, true) {
            return true;
        }

        let mut dampened_inputs = inputs.clone();
        dampened_inputs.remove(dampened_inputs.len() - 1);
        if is_report_safe(&dampened_inputs, true) {
            return true;
        }
    }

    return !issue_found;
}
