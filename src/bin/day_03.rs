use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_03_input.txt").unwrap();

    println!("{}", part_1(&input));
    // 476319 too low
    // 518219 too low
}

fn part_1(input: &str) -> u32 {
    // oof this looks tough
    // Numbers surrounded by all . are to be excluded
    // or numbers on the edge
    // numbers can share a symbol
    // 123*45
    // doesn't appear to be any adjacent numbers
    // none of this:
    // ..123..
    // ...456.

    // I think I should do this 3 lines at a time
    // don't need to put the whole thing into a grid at once

    // alternate
    // should I find symbols and then check around them?
    // There are fewer symbols than numbers
    // there are at most 4 numbers around each symbol
    // I might end up double counting numbers if a number touches 2 symbols
    // but I don't think any do touch 2 symbols

    let lines: Vec<&str> = input.lines().collect();

    let mut line_a: Vec<&str> = lines[0].split("").filter(|x| !x.is_empty()).collect();
    let mut line_b: Vec<&str> = lines[1]
        .split("")
        .filter(|x| !x.contains(" ") && !x.is_empty())
        .collect();
    let mut line_c: Vec<&str> = lines[2]
        .split("")
        .filter(|x| !x.contains(" ") && !x.is_empty())
        .collect();

    let mut empty_line: Vec<&str> = Vec::new();
    for _i in 0..line_a.len() {
        empty_line.push(".");
    }

    // println!("{:?}", empty_line);
    // println!("{:?}", line_a);
    // println!("{:?}", line_b);
    // println!("{:?}", line_c);
    // let mut sum = 0;

    // // test
    // sum += get_center_sum_from_lines(&line_a, &line_b, &line_c);
    // println!("sum 617 = {}", sum);
    // return sum;

    let mut sum = get_center_sum_from_lines(&empty_line, &line_a, &line_b);
    // println!("first sum {}", sum);
    // println!("sum 35 + 633 = {}", sum);

    for i in 1..lines.len() {
        match i % 3 {
            1 => {
                // abc case
                sum += get_center_sum_from_lines(&line_a, &line_b, &line_c);
                if i + 2 == lines.len() {
                    println!("finished at abc. i = {}", i);
                    sum += get_center_sum_from_lines(&line_b, &line_c, &empty_line);

                    break;
                }
                line_a = lines[i + 2]
                    .split("")
                    .filter(|x| !x.contains(" ") && !x.is_empty())
                    .collect();
            }
            2 => {
                // bca case
                sum += get_center_sum_from_lines(&line_b, &line_c, &line_a);
                if i + 2 == lines.len() {
                    println!("finished at bca. i = {}", i);
                    sum += get_center_sum_from_lines(&line_c, &line_a, &empty_line);

                    break;
                }
                line_b = lines[i + 2]
                    .split("")
                    .filter(|x| !x.contains(" ") && !x.is_empty())
                    .collect();
            }
            0 => {
                // cab case
                sum += get_center_sum_from_lines(&line_c, &line_a, &line_b);
                if i + 2 == lines.len() {
                    println!("finished at cab. i = {}", i);
                    sum += get_center_sum_from_lines(&line_a, &line_b, &empty_line);

                    break;
                }
                line_c = lines[i + 2]
                    .split("")
                    .filter(|x| !x.contains(" ") && !x.is_empty())
                    .collect();
            }
            _ => print!("broken"),
        };
    }

    sum
}

fn get_center_sum_from_lines(top: &Vec<&str>, mid: &Vec<&str>, bot: &Vec<&str>) -> u32 {
    // given 3 lines of &str, what is the sum of the center numbers
    let mut sum = 0;
    let mut num_stack = [0, 0, 0]; // numbers are 3 digits max
    let mut stack_count = 0;
    for i in 0..mid.len() {
        let mut num_ended = false;
        let mut num_on_right_edge = false;
        if let Ok(num) = mid[i].parse::<u32>() {
            num_stack[stack_count] = num;
            stack_count += 1;
            if i == (mid.len()-1) {
                num_ended = true;
                num_on_right_edge = true;
                // I missed the right edge originally
            }
        } else if stack_count > 0 {
            num_ended = true;
        }
        
        if num_ended {
            // number ended
            // check around for a symbol
            //.....  ...
            //.123.  .1.
            //.....  ...
            // count 3, count 1
            // need to check this area
            let mut found_symbol = false;
            if is_symbol(mid[i]) {
                found_symbol = true;
            } else if num_on_right_edge && is_symbol(mid[i - stack_count]){
                found_symbol = true;
            } else if (i - stack_count) > 0 && is_symbol(mid[i - stack_count - 1]) {
                found_symbol = true;
            } else {
                for j in 0..(stack_count + 2) {
                    let mut index = i - stack_count + j;
                    if num_on_right_edge {
                        if j == (stack_count + 1) {
                            break;
                        }
                    }
                    // i = 3, stack = 3
                    // want -1, 0, 1, 2, 3
                    // 3 - 3 + 0 - 1 = -1
                    // 3 - 3 + 1 - 1 = 0
                    if index > 0 {
                        if num_on_right_edge {
                            index += 1;
                        }
                        if is_symbol(top[index - 1]) {
                            found_symbol = true;
                            break;
                        } else if is_symbol(bot[index - 1]) {
                            found_symbol = true;
                            break;
                        }
                    }
                }
            }
            if found_symbol {
                // add the number
                // if the stack count is 3
                // [1,2,3] = 123
                // if the stack count is 1
                // [1] = 1
                match stack_count {
                    3 => sum += num_stack[0] * 100 + num_stack[1] * 10 + num_stack[2],
                    2 => sum += num_stack[0] * 10 + num_stack[1],
                    1 => sum += num_stack[0],
                    _ => (),
                };
                // println!("{}", sum);
            }
            stack_count = 0;
        }
    }

    sum
}

fn is_symbol(check: &str) -> bool {
    // let mut symbol = false;

    match check {
        "0" => false,
        "1" => false,
        "2" => false,
        "3" => false,
        "4" => false,
        "5" => false,
        "6" => false,
        "7" => false,
        "8" => false,
        "9" => false,
        "." => false,
        &_ => true,
    }

    // match check {
    //     "*" => symbol = true,
    //     "#" => symbol = true,
    //     "+" => symbol = true,
    //     "$" => symbol = true,
    //     "%" => symbol = true,
    //     "=" => symbol = true,
    //     "@" => symbol = true,
    //     "/" => symbol = true,
    //     "-" => symbol = true,
    //     "&" => symbol = true,

    //     &_ => (),
    // };
    // symbol
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_3: &str = "467..114..
    ...*......
    ..35..#632
    .....1....
    617.......
    ...*.+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn part_1_works() {
        assert_eq!(4361, part_1(&BASIC_INPUT_DAY_3));
    }
}
