use std::io;
use std::io::BufRead;

pub fn get_input_lines() -> Vec<String> {
    let stdin = io::stdin();

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

    return lines;
}
