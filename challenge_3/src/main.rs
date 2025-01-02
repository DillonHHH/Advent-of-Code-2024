mod custom_utils;

fn main() {
    let lines = custom_utils::get_input_lines();

    let mut result_stack: Vec<i32> = Vec::new();

    enum MulStages {
        None,
        M,
        U,
        L,
        OpenParentheses,
        Num1,
        Comma,
        Num2,
    }

    enum DoStages {
        None,
        D,
        O,
        OpenParentheses,
    }

    enum DontStages {
        None,
        D,
        O,
        N,
        Apostraphe,
        T,
        OpenParentheses,
    }

    let mut mul_stage = MulStages::None;
    let mut do_stage = DoStages::None;
    let mut dont_stage = DontStages::None;
    let mut do_mul = true;
    let mut num1_string: String = String::new();
    let mut num2_string: String = String::new();
    for line in lines {
        for char in line.chars() {
            match do_stage {
                DoStages::None => {
                    if char == 'd' {
                        do_stage = DoStages::D;
                    }
                }
                DoStages::D => {
                    if char == 'o' {
                        do_stage = DoStages::O;
                    } else {
                        do_stage = DoStages::None;
                    }
                }
                DoStages::O => {
                    if char == '(' {
                        do_stage = DoStages::OpenParentheses;
                    } else {
                        do_stage = DoStages::None;
                    }
                }
                DoStages::OpenParentheses => {
                    if char == ')' {
                        do_mul = true;
                    }
                    do_stage = DoStages::None;
                }
            }

            match dont_stage {
                DontStages::None => {
                    if char == 'd' {
                        dont_stage = DontStages::D;
                    }
                }
                DontStages::D => {
                    if char == 'o' {
                        dont_stage = DontStages::O;
                    } else {
                        dont_stage = DontStages::None;
                    }
                }
                DontStages::O => {
                    if char == 'n' {
                        dont_stage = DontStages::N;
                    } else {
                        dont_stage = DontStages::None;
                    }
                }
                DontStages::N => {
                    if char == '\'' {
                        dont_stage = DontStages::Apostraphe;
                    } else {
                        dont_stage = DontStages::None;
                    }
                }
                DontStages::Apostraphe => {
                    if char == 't' {
                        dont_stage = DontStages::T;
                    } else {
                        dont_stage = DontStages::None;
                    }
                }
                DontStages::T => {
                    if char == '(' {
                        dont_stage = DontStages::OpenParentheses;
                    } else {
                        dont_stage = DontStages::None;
                    }
                }
                DontStages::OpenParentheses => {
                    if char == ')' {
                        do_mul = false;
                    }
                    dont_stage = DontStages::None;
                }
            }

            match mul_stage {
                MulStages::None => {
                    if char == 'm' {
                        mul_stage = MulStages::M;
                    }
                }
                MulStages::M => {
                    if char == 'u' {
                        mul_stage = MulStages::U;
                    } else {
                        mul_stage = MulStages::None;
                    }
                }
                MulStages::U => {
                    if char == 'l' {
                        mul_stage = MulStages::L;
                    } else {
                        mul_stage = MulStages::None;
                    }
                }
                MulStages::L => {
                    if char == '(' {
                        mul_stage = MulStages::OpenParentheses;
                    } else {
                        mul_stage = MulStages::None;
                    }
                }
                MulStages::OpenParentheses => {
                    if char >= '0' && char <= '9' {
                        // reset the num strings before pushing new numbers into them
                        num1_string = String::new();
                        num2_string = String::new();

                        num1_string.push(char);
                        mul_stage = MulStages::Num1;
                    } else {
                        mul_stage = MulStages::None;
                    }
                }
                MulStages::Num1 => {
                    if char >= '0' && char <= '9' {
                        num1_string.push(char);
                    } else if char == ',' {
                        mul_stage = MulStages::Comma;
                    } else {
                        mul_stage = MulStages::None;
                    }
                }
                MulStages::Comma => {
                    if char >= '0' && char <= '9' {
                        num2_string.push(char);
                        mul_stage = MulStages::Num2;
                    } else {
                        mul_stage = MulStages::None;
                    }
                }
                MulStages::Num2 => {
                    if char >= '0' && char <= '9' {
                        num2_string.push(char);
                    } else if char == ')' {
                        mul_stage = MulStages::None;
                        if do_mul {
                            result_stack.push(
                                num1_string.parse::<i32>().unwrap()
                                    * num2_string.parse::<i32>().unwrap(),
                            );
                        }
                    } else {
                        mul_stage = MulStages::None;
                    }
                }
            }
        }
    }

    let mut final_result = 0;

    for result in result_stack {
        final_result += result;
    }

    println!("Final Result: {}", final_result);
}
