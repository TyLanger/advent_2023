use std::fs;

// run with cargo run --bin day_1_part_1
fn main() {
    let input = fs::read_to_string("./inputs/day_01_input.txt").unwrap();

    println!("{}", part_1(&input));
    // 17094737 too high
    // wasn't using single digits
    // a1kdfsjl222222klsd
    // should be [1, 2], not [1, 222222]
    // 54597 correct
    println!("{}", part_2(&input));
    // 54504 correct
}

fn part_1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for item in lines {
        sum += get_line_number(item);
    }

    sum
}

fn part_2(input: &str) -> u32 {
    // need to check for words
    // one, two, three, etc

    // given the split, is it a number? yes, use that
    // if no, check for a word
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for item in lines {
        sum += get_line_number_including_words(item);
    }
    sum
}

fn get_line_number(line: &str) -> u32 {
    let split: Vec<&str> = line.split("").collect();
    let mut sum = 0;
    // forwards
    for item in &split {
        if let Ok(num) = item.parse::<u32>() {
            // I don't need to figure out how to stitch the numbers together
            // just the first number x 10
            sum += num * 10;
            break;
        }
    }
    for i in 0..split.len() {
        // loop in reverse
        let item = split[split.len() - 1 - i];
        if let Ok(num) = item.parse::<u32>() {
            sum += num;
            break;
        }
    }

    sum
}

fn get_line_number_including_words(line: &str) -> u32 {
    // efficiencies:
    // when it finds something, it stops so it never goes through the whole array
    // inefficiencies
    // it checks the next ~5 places, ~10 times for each iteration
    // ex. i=0
    // it will check split[0] for a number
    // then it will check split[0], split[1], split[2] for "one"
    // then it will check the same 3 for "two"
    // I could have it quit early if it doesn't find a correct letter
    // in one, two, three, four, five, six, seven, eight
    // there is no q, y, p, d, j, etc.
    // so I could quit out early on those letters
    // would this actually save any time? probably not really

    // alternate solutions
    // seems what other people were doing was find and replace
    // abcnineabc -> abc9abc
    // they were having trouble with oneight
    // should be 18, but they would get 1ight with the e getting cut off
    let split: Vec<&str> = line.split("").filter(|x| !x.is_empty()).collect();
    let mut sum = 0;
    for i in 0..split.len() {
        // is it a number?
        if let Ok(num) = split[i].parse::<u32>() {
            sum += num * 10;
            break;
        } else if (split.len() - i >= 3)
            && split[i] == "o"
            && split[i + 1] == "n"
            && split[i + 2] == "e"
        {
            sum += 10;
            break;
            // println!("one found");
            // bounds
            // [o, n, e] len = 3
            // i = 0
            // len - i >= 3
        } else if (split.len() - i >= 3)
            && split[i] == "t"
            && split[i + 1] == "w"
            && split[i + 2] == "o"
        {
            sum += 20;
            break;
        } else if (split.len() - i >= 5)
            && split[i] == "t"
            && split[i + 1] == "h"
            && split[i + 2] == "r"
            && split[i + 3] == "e"
            && split[i + 4] == "e"
        {
            sum += 30;
            break;
        } else if (split.len() - i >= 4)
            && split[i] == "f"
            && split[i + 1] == "o"
            && split[i + 2] == "u"
            && split[i + 3] == "r"
        {
            sum += 40;
            break;
        } else if (split.len() - i >= 4)
            && split[i] == "f"
            && split[i + 1] == "i"
            && split[i + 2] == "v"
            && split[i + 3] == "e"
        {
            sum += 50;
            break;
        } else if (split.len() - i >= 3)
            && split[i] == "s"
            && split[i + 1] == "i"
            && split[i + 2] == "x"
        {
            sum += 60;
            break;
        } else if (split.len() - i >= 5)
            && split[i] == "s"
            && split[i + 1] == "e"
            && split[i + 2] == "v"
            && split[i + 3] == "e"
            && split[i + 4] == "n"
        {
            sum += 70;
            break;
        } else if (split.len() - i >= 5)
            && split[i] == "e"
            && split[i + 1] == "i"
            && split[i + 2] == "g"
            && split[i + 3] == "h"
            && split[i + 4] == "t"
        {
            sum += 80;
            break;
        } else if (split.len() - i >= 4)
            && split[i] == "n"
            && split[i + 1] == "i"
            && split[i + 2] == "n"
            && split[i + 3] == "e"
        {
            sum += 90;
            break;
        }
    }

    for j in 0..split.len() {
        // run in reverse
        // transform to i so I can copy/paste
        let i = split.len() - 1 - j;
        // is it a number?
        if let Ok(num) = split[i].parse::<u32>() {
            sum += num;
            // println!("found a number");
            break;
        } else if (split.len() - i >= 3)
            && split[i] == "o"
            && split[i + 1] == "n"
            && split[i + 2] == "e"
        {
            sum += 1;
            break;
        } else if (split.len() - i >= 3)
            && split[i] == "t"
            && split[i + 1] == "w"
            && split[i + 2] == "o"
        {
            sum += 2;
            break;
        } else if (split.len() - i >= 5)
            && split[i] == "t"
            && split[i + 1] == "h"
            && split[i + 2] == "r"
            && split[i + 3] == "e"
            && split[i + 4] == "e"
        {
            sum += 3;
            break;
        } else if (split.len() - i >= 4)
            && split[i] == "f"
            && split[i + 1] == "o"
            && split[i + 2] == "u"
            && split[i + 3] == "r"
        {
            sum += 4;
            break;
        } else if (split.len() - i >= 4)
            && split[i] == "f"
            && split[i + 1] == "i"
            && split[i + 2] == "v"
            && split[i + 3] == "e"
        {
            sum += 5;
            break;
        } else if (split.len() - i >= 3)
            && split[i] == "s"
            && split[i + 1] == "i"
            && split[i + 2] == "x"
        {
            sum += 6;
            break;
        } else if (split.len() - i >= 5)
            && split[i] == "s"
            && split[i + 1] == "e"
            && split[i + 2] == "v"
            && split[i + 3] == "e"
            && split[i + 4] == "n"
        {
            sum += 7;
            break;
        } else if (split.len() - i >= 5)
            && split[i] == "e"
            && split[i + 1] == "i"
            && split[i + 2] == "g"
            && split[i + 3] == "h"
            && split[i + 4] == "t"
        {
            sum += 8;
            break;
        } else if (split.len() - i >= 4)
            && split[i] == "n"
            && split[i + 1] == "i"
            && split[i + 2] == "n"
            && split[i + 3] == "e"
        {
            sum += 9;
            break;
        }
    }

    // println!("{}", sum);
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_01: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const BASIC_INPUT_DAY_01_PART_2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn part_1_works() {
        assert_eq!(142, part_1(&BASIC_INPUT_DAY_01));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(281, part_2(&BASIC_INPUT_DAY_01_PART_2));
    }

    #[test]
    fn letter_checking() {
        // checking if this works how I think it does
        let line = "two1nine";
        let split: Vec<&str> = line.split("").filter(|x| !x.is_empty()).collect();
        assert_eq!("t", split[0]);
        // I can also do this
        assert_eq!("two", &line[..3]);
        assert_eq!("nine", &line[4..]);
    }
}
