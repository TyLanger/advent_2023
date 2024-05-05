use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_04_input.txt").unwrap();

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for line in lines {
        let split: Vec<&str> = line.split("|").collect();
        let mut winners: Vec<u32> = split[0]
            .split(" ")
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();
        let mut numbers: Vec<u32> = split[1]
            .split(" ")
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        winners.sort();
        numbers.sort();

        // print!("Winners: {:?}", &winners);
        // println!("  Numbers: {:?}", &numbers);

        let mut num_hits = 0;
        // when checking a new winning number, start where you left off
        // in the number we have. They are sorted after all.
        let mut number_index = 0;
        for w in winners {
            for i in number_index..numbers.len() {
                if w == numbers[i] {
                    num_hits += 1;
                    number_index = i;
                    // assuming no duplicates in the numbers we have
                    break;
                } else if w < numbers[i] {
                    number_index = i;
                    break;
                }
            }
        }
        sum += match num_hits {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 8,
            5 => 16,
            6 => 32,
            7 => 64,
            8 => 128,
            9 => 256,
            10 => 512,
            11 => 1024,
            _ => {
                println!("hit 12+ hits. Shouldn't be possible");
                0
            }
        };
    }

    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_04: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part_1_works() {
        assert_eq!(13, part_1(&BASIC_INPUT_DAY_04));
    }
}
