use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_02_input.txt").unwrap();

    println!("{}", part_1(&input));
    // 2278 correct
    println!("{}", part_2(&input));
    // 67953 correct
}

fn part_1(input: &str) -> u32 {
    // which games are possible?
    // assume the bag contains:
    let red = 12;
    let green = 13;
    let blue = 14;

    let min = 12;

    // the game is impossible if the elf reveals too many of a colour
    // like if they reveal 20 red cubes at once

    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    let mut count = 0;

    for line in lines {
        let split: Vec<&str> = line.split(' ').filter(|x| !x.is_empty()).collect();
        // [Game 1: 3 blue, 4 red; ....]
        // split[2] should be the first number

        // count is the ID of this game
        count += 1;

        let mut valid_game = true;

        // check a number and its associated colour
        // if the number is smaller than the smallest of cubes (12), can skip it
        for i in 2..split.len() {
            if (i - 1) % 2 == 0 {
                // skip the odd indicies. They are "blue", "red"
                // numbers are the even indicies
                continue;
            }
            // println!("{:?}", split[i]);
            if let Ok(num) = split[i].parse::<u32>() {
                if num < min {
                    // lower than all. Don't care about the colour
                    // speeds up the loop. Does this actually work
                    // i += 1;
                } else if &split[i + 1][..3] == "red" {
                    if num > red {
                        // fake game
                        valid_game = false;
                        break;
                    }
                } else if &split[i + 1][..4] == "blue" {
                    if num > blue {
                        // fake game
                        valid_game = false;
                        break;
                    }
                } else if &split[i + 1][..5] == "green" {
                    if num > green {
                        // fake game
                        valid_game = false;
                        break;
                    }
                }
            }
        }
        if valid_game {
            sum += count;
        }
    }

    // return sum of POSSIBLE IDs
    sum
}

fn part_2(input: &str) -> u32 {
    // find the largest of each colour in a game
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // 4 red, 2 green, 6 blue
    // then multipl them together. 4 x 2 x 6 = 48
    // sum all the powers and return

    // could I sort them?
    // get ["4 green", "2 green", "6 red", "1 red", "4 blue"]

    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for line in lines {
        // let split: Vec<&str> = line.split(' ').filter(|x| !x.is_empty()).collect();

        let split: Vec<&str> = line
            .split(&[':', ',', ';'])
            .collect();

        // sort didn't work
        // ["Game 1", " 3 blue", " 4 red", " 1 red", " 2 green", " 6 blue", " 2 green"]
        // split.sort();
        // [" 1 red", " 2 green", " 2 green", " 3 blue", " 4 red", " 6 blue", "Game 1"]

        // sort
        // when sorted, the largest numbers are to the right
        // need to just iterate until you find each colour and then you can quit
        // bc you know you have the biggest numbers

        // error
        // sorting breaks with double digits...
        // 14 blue, 6 blue
        // error 2
        // "    game 2", " 6 green"
        // game also gets sorted to the start on subsequent games...

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for i in 1..split.len() {
            // start at 1 to skip "game x"
            
            // turn " 6 blue" into:
            // ["", "6", "blue"]
            let second_split: Vec<&str> = split[i].split(' ').collect();
            // figure out what colour
            if second_split[2] == "red" {
                if let Ok(num) = second_split[1].parse::<u32>() {
                    if num > max_red {
                        max_red = num;
                    }
                }
            } else if second_split[2] == "blue" {
                if let Ok(num) = second_split[1].parse::<u32>() {
                    if num > max_blue {
                        max_blue = num;
                    }
                }
            } else if second_split[2] == "green" {
                if let Ok(num) = second_split[1].parse::<u32>() {
                    if num > max_green {
                        max_green = num;
                    }
                }
            }
        }

        sum += max_red * max_blue * max_green;
    }

    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_2: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1_works() {
        assert_eq!(8, part_1(&BASIC_INPUT_DAY_2));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(2286, part_2(&BASIC_INPUT_DAY_2));
    }

    #[test]
    fn colour_names() {
        // does it ignore the , or ; at the end?
        let red = "red,";
        assert_eq!("red", &red[..3]);

        let blue = "blue;";
        assert_eq!("blue", &blue[..4]);

        let green = "green";
        assert_eq!("green", &green[..5]);
    }

    #[test]
    fn sort_test() {
        let lines: Vec<&str> = BASIC_INPUT_DAY_2.lines().collect();
        let mut split: Vec<&str> = lines[0].split(&[':', ',', ';']).collect();
        split.sort();
        println!("{:?}", split);
        // before
        // ["Game 1", " 3 blue", " 4 red", " 1 red", " 2 green", " 6 blue", " 2 green"]
        // after
        // [" 1 red", " 2 green", " 2 green", " 3 blue", " 4 red", " 6 blue", "Game 1"]
        // I think I can work with this

        let second_split: Vec<&str> = split[5].split(' ').collect();
        println!("{:?}", second_split);
        // ["", "6", "blue"]
        assert_eq!(6, second_split[1].parse::<u32>().unwrap());
        // this case works, but other don't
    }
}
