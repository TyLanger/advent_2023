use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_04_input.txt").unwrap();

    println!("{}", part_1(&input));
    // 24175
    println!("{}", part_2(&input));
    // 18731808 too low
    // whoops, the copies array should be 11, not 10
    // needs to be 11 to fit the possible 10 matches
    // you are given 10 winnng numbers. It's possible they all match
    // 18846301 correct
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
        let num_hits = how_many_matches(&winners, &numbers);

        if num_hits > 0 {
            // can do doubles by shifting
            sum += 1 << (num_hits - 1);
        }
        // sum += match num_hits {
        //     0 => 0,
        //     1 => 1,
        //     2 => 2,
        //     3 => 4,
        //     4 => 8,
        //     5 => 16,
        //     6 => 32,
        //     7 => 64,
        //     8 => 128,
        //     9 => 256,
        //     10 => 512,
        //     11 => 1024,
        //     _ => {
        //         println!("hit 12+ hits. Shouldn't be possible");
        //         0
        //     }
        // };
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    // keep track of how many copies of the associated card you have
    // only need to keep track of the next 10, don't need to remember
    // how many copies of all 200+ cards in the input.
    // 11 because current + 10 cards ahead. It's winners.len() + 1
    let mut copies = vec![0; 11];
    let mut copy_index: usize = 0;

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

        let num_hits = how_many_matches(&winners, &numbers);
        let old_index = copy_index;
        copy_index = (copy_index + 1) % copies.len();

        // the next (num_hits) cards get 1 extra card
        for i in 0..num_hits {
            let overlap: usize = (copy_index + usize::try_from(i).unwrap()) % copies.len();
            copies[overlap] += 1 + copies[old_index];
            sum += 1 + copies[old_index];
        }
        // count the real card
        sum += 1;
        // reset and reuse this circular array
        copies[old_index] = 0;
    }

    sum
}

fn how_many_matches(winners: &Vec<u32>, numbers: &Vec<u32>) -> u32 {
    let mut num_hits = 0;
    // when checking a new winning number, start where you left off
    // in the number we have. They are sorted after all.
    let mut number_index = 0;
    for w in winners {
        for i in number_index..numbers.len() {
            if *w == numbers[i] {
                num_hits += 1;
                number_index = i;
                // assuming no duplicates in the numbers we have
                break;
            } else if *w < numbers[i] {
                number_index = i;
                break;
            }
        }
    }

    num_hits
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

    #[test]
    fn part_2_works() {
        assert_eq!(30, part_2(&BASIC_INPUT_DAY_04));
    }

    #[test]
    fn modulo_math() {
        assert_eq!(3, 3 % 10);
        assert_eq!(3, 13 % 10);
        assert_eq!(0, 10 % 10);
    }
}
