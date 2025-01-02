mod custom_utils;

fn main() {
    let lines = custom_utils::get_input_lines();

    let mut word_search: Vec<Vec<char>> = Vec::new();

    for line in lines {
        word_search.push(line.chars().collect());
    }

    let mut count = 0;

    for line_index in 0..word_search.len() {
        for char_index in 0..word_search[line_index].len() {
            if word_search[line_index][char_index] == 'M' {
                // NE
                if line_index >= 2 && word_search[line_index].len() - char_index - 1 >= 2 {
                    if word_search[line_index - 1][char_index + 1] == 'A'
                        && word_search[line_index - 2][char_index + 2] == 'S'
                    {
                        if (word_search[line_index - 2][char_index] == 'M'
                            && word_search[line_index][char_index + 2] == 'S')
                            || (word_search[line_index - 2][char_index] == 'S'
                                && word_search[line_index][char_index + 2] == 'M')
                        {
                            count += 1;
                            word_search[line_index][char_index] = '-'; // removing the m ensures no X will be counted twice
                        }
                    }
                }

                // SE
                if word_search.len() - line_index - 1 >= 2
                    && word_search[line_index].len() - char_index - 1 >= 2
                {
                    if word_search[line_index + 1][char_index + 1] == 'A'
                        && word_search[line_index + 2][char_index + 2] == 'S'
                    {
                        if (word_search[line_index + 2][char_index] == 'M'
                            && word_search[line_index][char_index + 2] == 'S')
                            || (word_search[line_index + 2][char_index] == 'S'
                                && word_search[line_index][char_index + 2] == 'M')
                        {
                            count += 1;
                            word_search[line_index][char_index] = '-';
                        }
                    }
                }

                // SW
                if word_search.len() - line_index - 1 >= 2 && char_index >= 2 {
                    if word_search[line_index + 1][char_index - 1] == 'A'
                        && word_search[line_index + 2][char_index - 2] == 'S'
                    {
                        if (word_search[line_index + 2][char_index] == 'M'
                            && word_search[line_index][char_index - 2] == 'S')
                            || (word_search[line_index + 2][char_index] == 'S'
                                && word_search[line_index][char_index - 2] == 'M')
                        {
                            count += 1;
                            word_search[line_index][char_index] = '-';
                        }
                    }
                }

                // NW
                if line_index >= 2 && char_index >= 2 {
                    if word_search[line_index - 1][char_index - 1] == 'A'
                        && word_search[line_index - 2][char_index - 2] == 'S'
                    {
                        if (word_search[line_index - 2][char_index] == 'M'
                            && word_search[line_index][char_index - 2] == 'S')
                            || (word_search[line_index - 2][char_index] == 'S'
                                && word_search[line_index][char_index - 2] == 'M')
                        {
                            count += 1;
                            word_search[line_index][char_index] = '-';
                        }
                    }
                }
            }
        }
    }

    println!("XMAS Count: {}", count);
}

// part 1 solution
// fn main() {
//     let lines = custom_utils::get_input_lines();

//     let mut word_search: Vec<Vec<char>> = Vec::new();

//     for line in lines {
//         word_search.push(line.chars().collect());
//     }

//     let mut count = 0;

//     for (line_index, line) in word_search.iter().enumerate() {
//         for (char_index, &char) in line.iter().enumerate() {
//             if char == 'X' {
//                 // N
//                 if line_index >= 3 {
//                     if word_search[line_index - 1][char_index] == 'M'
//                         && word_search[line_index - 2][char_index] == 'A'
//                         && word_search[line_index - 3][char_index] == 'S'
//                     {
//                         println!("N Found");
//                         count += 1;
//                     }
//                 }

//                 // NE
//                 if line_index >= 3 && line.len() - char_index - 1 >= 3 {
//                     if word_search[line_index - 1][char_index + 1] == 'M'
//                         && word_search[line_index - 2][char_index + 2] == 'A'
//                         && word_search[line_index - 3][char_index + 3] == 'S'
//                     {
//                         println!("NE Found");
//                         count += 1;
//                     }
//                 }

//                 // E
//                 if line.len() - char_index - 1 >= 3 {
//                     if word_search[line_index][char_index + 1] == 'M'
//                         && word_search[line_index][char_index + 2] == 'A'
//                         && word_search[line_index][char_index + 3] == 'S'
//                     {
//                         println!("E Found");
//                         count += 1;
//                     }
//                 }

//                 // SE
//                 if word_search.len() - line_index - 1 >= 3 && line.len() - char_index - 1 >= 3 {
//                     if word_search[line_index + 1][char_index + 1] == 'M'
//                         && word_search[line_index + 2][char_index + 2] == 'A'
//                         && word_search[line_index + 3][char_index + 3] == 'S'
//                     {
//                         println!("SE Found");
//                         count += 1;
//                     }
//                 }

//                 // S
//                 if word_search.len() - line_index - 1 >= 3 {
//                     if word_search[line_index + 1][char_index] == 'M'
//                         && word_search[line_index + 2][char_index] == 'A'
//                         && word_search[line_index + 3][char_index] == 'S'
//                     {
//                         println!("S Found");
//                         count += 1;
//                     }
//                 }

//                 // SW

//                 if word_search.len() - line_index - 1 >= 3 && char_index >= 3 {
//                     if word_search[line_index + 1][char_index - 1] == 'M'
//                         && word_search[line_index + 2][char_index - 2] == 'A'
//                         && word_search[line_index + 3][char_index - 3] == 'S'
//                     {
//                         println!("SW Found");
//                         count += 1;
//                     }
//                 }

//                 // W
//                 if char_index >= 3 {
//                     if word_search[line_index][char_index - 1] == 'M'
//                         && word_search[line_index][char_index - 2] == 'A'
//                         && word_search[line_index][char_index - 3] == 'S'
//                     {
//                         println!("W Found");
//                         count += 1;
//                     }
//                 }

//                 // NW
//                 if line_index >= 3 && char_index >= 3 {
//                     if word_search[line_index - 1][char_index - 1] == 'M'
//                         && word_search[line_index - 2][char_index - 2] == 'A'
//                         && word_search[line_index - 3][char_index - 3] == 'S'
//                     {
//                         println!("NW Found");
//                         count += 1;
//                     }
//                 }
//             }
//         }
//     }

//     println!("XMAS Count: {}", count);
// }
