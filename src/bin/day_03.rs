use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_03_input.txt").unwrap();

    println!("{}", part_1(&input));
    // 476319 too low
    // 518219 too low
    // I missed the numbers on the right edge case: ...*123
    // 522726 correct
    println!("{}", part_2(&input));
    // 81721933 correct
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

    let mut sum = get_center_sum_from_lines(&empty_line, &line_a, &line_b);

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

fn part_2(input: &str) -> u32 {
    // Notes:
    // need to find gears
    // 123*45
    // ....
    // ..123
    // ...*.
    // ..456
    // It might be best to look for *s
    // and nearby numbers
    // which is the way I didn't do part 1
    // and the harder way...

    // cases
    // 123.567
    // ...*...
    // 123.567

    // these are the places I need to check
    // check the 3x3 around the * and if you find a number,
    // follow it to get the rest of it

    // shortcut
    // make sure there are 2 different numbers around the 3x3
    // before looking for the rest of the first number
    // 123..
    // ...*.
    // ....4
    // find 3, but don't search for 12 until you know 4 exists
    // getting clever like this is probablymore trouble than its worth
    // but I do need a way to figure out if there are 2 separate numbers
    // 123   1..   1.1
    // .*.   .*.   .*.
    // ...   ...   ...
    // I can see this being trickier than it looks

    // with 3 digit numbers, there are 6 cases for the cornersof the 3x3
    // 123..  12..  1..  123.  123  12.
    // ...*.  ..*.  .*.  ..*.  .*.  .*.
    // that would make for some messy nested if statements

    // This code is the same as part 1
    // just swap get_center_sum_from_lines() for get_gear_ratio_from_lines()
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

    let mut sum = get_gear_ratio_from_lines(&empty_line, &line_a, &line_b);

    for i in 1..lines.len() {
        match i % 3 {
            1 => {
                sum += get_gear_ratio_from_lines(&line_a, &line_b, &line_c);
                if i + 2 == lines.len() {
                    println!("finished at abc. i = {}", i);
                    sum += get_gear_ratio_from_lines(&line_b, &line_c, &empty_line);

                    break;
                }
                line_a = lines[i + 2]
                    .split("")
                    .filter(|x| !x.contains(" ") && !x.is_empty())
                    .collect();
            }
            2 => {
                // bca case
                sum += get_gear_ratio_from_lines(&line_b, &line_c, &line_a);
                if i + 2 == lines.len() {
                    println!("finished at bca. i = {}", i);
                    sum += get_gear_ratio_from_lines(&line_c, &line_a, &empty_line);

                    break;
                }
                line_b = lines[i + 2]
                    .split("")
                    .filter(|x| !x.contains(" ") && !x.is_empty())
                    .collect();
            }
            0 => {
                // cab case
                sum += get_gear_ratio_from_lines(&line_c, &line_a, &line_b);
                if i + 2 == lines.len() {
                    println!("finished at cab. i = {}", i);
                    sum += get_gear_ratio_from_lines(&line_a, &line_b, &empty_line);

                    break;
                }
                line_c = lines[i + 2]
                    .split("")
                    .filter(|x| !x.contains(" ") && !x.is_empty())
                    .collect();
            }
            _ => (),
        };
    }

    sum
}

fn get_gear_ratio_from_lines(top: &Vec<&str>, mid: &Vec<&str>, bot: &Vec<&str>) -> u32 {
    // try to find a *
    let mut sum = 0;
    for i in 0..mid.len() {
        if mid[i] == "*" {
            let top_numbers = get_numbers_from_line(&top, i);
            let mid_numbers = get_numbers_from_line(&mid, i);
            let bot_numbers = get_numbers_from_line(&bot, i);

            let mut the_numbers = Vec::new();
            if let Some(num) = top_numbers.0 {
                the_numbers.push(num);
            }
            if let Some(num) = top_numbers.1 {
                the_numbers.push(num);
            }
            if let Some(num) = mid_numbers.0 {
                the_numbers.push(num);
            }
            if let Some(num) = mid_numbers.1 {
                the_numbers.push(num);
            }
            if let Some(num) = bot_numbers.0 {
                the_numbers.push(num);
            }
            if let Some(num) = bot_numbers.1 {
                the_numbers.push(num);
            }

            // only update sum if exactly 2 numbers
            // I don't think the input has any cases with 3 adjacent numbers, but oh well
            if the_numbers.len() == 2 {
                sum += the_numbers[0] * the_numbers[1];
            }
        }
    }

    sum
}

fn get_numbers_from_line(line: &Vec<&str>, index: usize) -> (Option<u32>, Option<u32>) {
    let i = index;
    // cases
    // can be 0, 1, 2 numbers per line
    // can only be 2 numbers if the center is empty
    let mut all_numbers = [None; 7];
    if i > 0 {
        if let Ok(left) = line[i - 1].parse::<u32>() {
            // left contains a number
            all_numbers[2] = Some(left);
        }
    }
    if let Ok(center) = line[i].parse::<u32>() {
        all_numbers[3] = Some(center);
    }
    if i < (line.len() - 1) {
        if let Ok(right) = line[i + 1].parse::<u32>() {
            all_numbers[4] = Some(right);
        }
    }

    // all 3 failed
    // return no numbers
    // ...
    // .*.
    if all_numbers[2].is_none() && all_numbers[3].is_none() && all_numbers[4].is_none() {
        return (None, None);
    }

    // 123
    // .*.
    if all_numbers[2].is_some() && all_numbers[3].is_some() && all_numbers[4].is_some() {
        // 3 digit number centered
        let sum =
            all_numbers[2].unwrap() * 100 + all_numbers[3].unwrap() * 10 + all_numbers[4].unwrap();
        return (Some(sum), None);
    }

    if all_numbers[2].is_some() && i > 1 {
        // check left of it
        if let Ok(leftleft) = line[i - 2].parse::<u32>() {
            all_numbers[1] = Some(leftleft);

            if i > 2 {
                // check leftleftleft
                if let Ok(left3) = line[i - 3].parse::<u32>() {
                    all_numbers[0] = Some(left3);
                }
            }
        }
    }

    if all_numbers[4].is_none() {
        // found 1 left aligned number
        // ????x
        // ...*.
        let mut sum = 0;
        let mut multiplier = 1;
        for j in 0..4 {
            if let Some(num) = all_numbers[3 - j] {
                sum += num * multiplier;
                multiplier *= 10;
            }
        }
        // 1..   12..   123..
        // .*.   ..*.   ...*.
        return (Some(sum), None);
    } else {
        // check right
        if i < line.len() - 2 {
            if let Ok(rightright) = line[i + 2].parse::<u32>() {
                all_numbers[5] = Some(rightright);

                if i < line.len() - 3 {
                    if let Ok(right3) = line[i + 3].parse::<u32>() {
                        all_numbers[6] = Some(right3);
                    }
                }
            }
        }
    }

    // 2 numbers
    // can only be 2 numbers if the center is empty
    // 123.123   12.12   1.1
    // ...*...   ..*..   .*.
    if all_numbers[2].is_some() && all_numbers[3].is_none() && all_numbers[4].is_some() {
        let mut left_sum = 0;
        let mut multiplier = 1;
        for j in 0..4 {
            if let Some(num) = all_numbers[3 - j] {
                left_sum += num * multiplier;
                multiplier *= 10;
            }
        }

        let mut right_sum = 0;
        let mut multiplier = 1;
        for j in 0..4 {
            if let Some(num) = all_numbers[6 - j] {
                right_sum += num * multiplier;
                multiplier *= 10;
            }
        }
        return (Some(left_sum), Some(right_sum));
    }

    // last case: one number right aligned
    let mut right_sum = 0;
    let mut multiplier = 1;
    for j in 0..4 {
        if let Some(num) = all_numbers[6 - j] {
            right_sum += num * multiplier;
            multiplier *= 10;
        }
    }
    // ..1   ...12   ..123
    // .*.   ..*.    .*.
    (Some(right_sum), None)
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
            if i == (mid.len() - 1) {
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
            } else if num_on_right_edge && is_symbol(mid[i - stack_count]) {
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

    const BASIC_INPUT_DAY_3_PT_2: &str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn part_1_works() {
        assert_eq!(4361, part_1(&BASIC_INPUT_DAY_3));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(467835, part_2(&BASIC_INPUT_DAY_3_PT_2));
    }

    #[test]
    fn test_number_finding() {
        let test_line = vec!["1", "2", "3"];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((Some(123), None), a);
        let test_line = vec![".", ".", "."];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((None, None), a);

        let test_line = vec!["1", "2", "."];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((Some(12), None), a);
        let test_line = vec![".", "1", "2"];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((Some(12), None), a);

        let test_line = vec!["1", ".", "."];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((Some(1), None), a);
        let test_line = vec![".", "1", "."];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((Some(1), None), a);
        let test_line = vec![".", ".", "1"];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((Some(1), None), a);

        let test_line = vec!["1", ".", "1"];
        let a = get_numbers_from_line(&test_line, 1);
        assert_eq!((Some(1), Some(1)), a);

        let test_line = vec!["1", "2", "3", "."];
        let a = get_numbers_from_line(&test_line, 2);
        assert_eq!((Some(123), None), a);

        let test_line = vec!["1", "2", "3", ".", ".", ".", "."];
        let a = get_numbers_from_line(&test_line, 3);
        assert_eq!((Some(123), None), a);
        let test_line = vec![".", ".", ".", ".", "1", "2", "3"];
        let a = get_numbers_from_line(&test_line, 3);
        assert_eq!((Some(123), None), a);

        let test_line = vec!["1", "2", "3", ".", "1", "2", "3"];
        let a = get_numbers_from_line(&test_line, 3);
        assert_eq!((Some(123), Some(123)), a);
        let test_line = vec![".", "2", "3", ".", "1", "2", "."];
        let a = get_numbers_from_line(&test_line, 3);
        assert_eq!((Some(23), Some(12)), a);
        // does it work if the star is in this line?
        let test_line = vec!["1", "2", "3", "*", "1", "2", "3"];
        let a = get_numbers_from_line(&test_line, 3);
        assert_eq!((Some(123), Some(123)), a);
        // this isn't all of the cases, but it is most of them
    }
}
