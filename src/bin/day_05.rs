use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_05_input.txt").unwrap();

    println!("{}", part_1(&input));
    // 1881211815 too high
    // whoops forgot to sort before output
    // 403695602 correct
    println!("{}", part_2(&input));
    // 39360041 too low
    // Wasn't handling values being equal correctly
    // 219529182 correct
}

#[derive(Debug)]
struct TransformMap {
    to: u64,
    from: u64,
    len: u64,
}

fn part_1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let mut seeds: Vec<u64> = lines[0]
        .split(" ")
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();

    // println!("Seeds: {:?}", &seeds);
    seeds.sort();
    // println!("Seeds sorted: {:?}", &seeds);

    // figure out the seed-soil map
    // all maps are technically the same so this should be a loop
    // when I find a blank line, stop and that's the full map

    let mut transform_maps: Vec<TransformMap> = Vec::new();
    let mut skip_line = 2;

    for i in 2..lines.len() {
        // is this line empty or is this the last line
        if lines[i].is_empty() || i == (lines.len() - 1) {
            // the maps are done, do the logic
            // println!("transform maps: {:?}", &transform_maps);
            seeds.sort();
            transform_maps.sort_by(|a, b| a.from.partial_cmp(&b.from).unwrap());
            // println!("transform maps after sort: {:?}", &transform_maps);
            let mut shortcut_index = 0;
            for j in 0..seeds.len() {
                for k in shortcut_index..transform_maps.len() {
                    if seeds[j] < transform_maps[k].from {
                        // number stays the same
                        break;
                    } else if seeds[j] < (transform_maps[k].from + transform_maps[k].len) {
                        // found the range to transform the seed
                        // 52 50 48 example
                        // to transform, take the old value + (52 - 50)
                        // 50 -> 52, 51 -> 53, ets
                        // 50 98 2 example
                        // 98 -> 50 or (98 + (50-98)), 99 -> 51
                        // let new_value = seeds[j] + (transform_maps[k].to - transform_maps[k].from);
                        // now how do I do it without subtraction?
                        // 50 + (98+2-1) - seed
                        // 39 0 15
                        // seed = 13
                        // want 39 + (13 - 0)
                        let new_value = transform_maps[k].to + seeds[j] - transform_maps[k].from;
                        // println!("change value: {} -> {}", seeds[j], new_value);
                        seeds[j] = new_value;
                        // ffs I missed this break
                        break;
                    } else {
                        shortcut_index = k + 1;
                        // shortcut_index = k;
                        // bigger than ths current map; check the next one
                        // update the shortcut_index so subsequent seed_numbers
                        // start closer to where they will end up
                        // all subsequent seed_numbers are larger than this one
                    }
                }
            }
            // I want to skip the next line
            // I know the next line after a blank will be
            // soil-to-fertilizer map:
            skip_line = i + 1;
            // clear the old maps
            transform_maps.clear();
            // println!("unsorted seeds: {:?}", &seeds);

            // seeds.sort();
            // println!("sorted seeds: {:?}", &seeds);
        } else if i == skip_line {
            // skip the "seel-to-soil map:" lines
            // they come after blank lines
            continue;
        } else {
            let triple: Vec<u64> = lines[i]
                .split(" ")
                .filter_map(|x| x.parse::<u64>().ok())
                .collect();
            let map = TransformMap {
                to: triple[0],
                from: triple[1],
                len: triple[2],
            };
            transform_maps.push(map);
        }
    }

    seeds.sort();
    // seed array is sorted so the lowest location is in seeds[0]
    // starts storing seeds, then soils, then fertilizer, etc.
    // all in the same seeds array
    seeds[0]
}

fn part_2(input: &str) -> u64 {
    // instead of having 4 seeds, I now have 27 seed numbers
    // can I still brute force this?
    // the real input will have maybe 10b seed numbers
    // there are 20 9-digit numbers
    // so 10 lengths of ~1b seeds

    // only need 2 seed numbers at each step
    // one that gets changed and another that doesn't
    // well, more than 2
    // If one transform is from 30 to 60,
    // I want to test what happens if I pass in 30 and what happens if I pass in 60
    // don't really need to check 31, 32, 33, etc
    // but then I need to do this minimax for each step
    // say step 1 is 30-60 gets turned into 80-110
    // then stp 2 is 90-95 gets turned to 5-10
    // computing for 30 and 60 doesn't get the min
    // I'd need to look ahead and do 90 to get to 5
    // this seems hard
    // how would I represent this?
    // I have seed numbers from 20-100
    // they can become 20-30, 30-60 becomes 80-110, 60-80, 80-110 becomes x-y
    // 90-95 -> 5-10
    // 20-30, 80-90, 90-95 becomes 5-10, 95-110, 60-80, x-y
    // seems possible
    // take a range from a-b, then possibly split it into a-c, c-d, d-b
    // maybe merge them?
    // I have 5-10, 15-30
    // next test is 0-40 becomes 70-110
    // Then I have 75-80, 85-100 with no merging?
    // merging is probably unlikely
    // double split
    // I have 40-80
    // next test is 45-50, 70-75
    // step 1: 40-45, 45-50, 50-80
    // step 2:               50-70, 70-75, 75-80
    // after: 40-45, 45-50, 50-70, 70-75, 75-80
    // how do I loop this?
    // how do I store the data?
    // 50-80 is an intermediary that needs to be checked twice. How do I determine these?

    // this is what I was thinking about in p1 about doing it backwards, starting with the location
    // I could start at 0 and then work backwards and test to see if there's a valid path to a seed
    // i would need to do 1b iterations probably. The solution for p1 was ~1b

    // can I add all the maps together?
    // form my own map
    // step 1 shifts 9 to 2
    // step 2 shifts 2 to 4
    // can be simplified to 9 to 4

    // seed ranges
    // [(a,b), (c,d), (e,f), etc]
    // go through each map and split these up

    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<u64> = lines[0]
        .split(" ")
        .filter_map(|x| x.parse::<u64>().ok())
        .collect();

    // inclusive
    // 50 98 2 would be 98,99 or 50,51
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let half_seeds = seeds.len() / 2;

    for i in 0..half_seeds {
        ranges.push((seeds[i * 2], seeds[i * 2] + seeds[i * 2 + 1] - 1));
    }

    ranges.sort();

    let mut transform_maps: Vec<TransformMap> = Vec::new();
    let mut skip_line = 2;

    let mut check_again: Vec<(u64, u64)> = Vec::new();

    for i in 2..lines.len() {
        // is this line empty or is this the last line
        if lines[i].is_empty() || i == (lines.len() - 1) {
            // the maps are done, do the logic
            transform_maps.sort_by(|a, b| a.from.partial_cmp(&b.from).unwrap());
            // println!("transform maps after sort: {:?}", &transform_maps);
            let mut shortcut_index = 0;
            let mut next_ranges: Vec<(u64, u64)> = Vec::new();

            for j in 0..ranges.len() {
                for k in shortcut_index..transform_maps.len() {
                    let mut a = ranges[j].0;
                    let mut b = ranges[j].1;
                    let c = transform_maps[k].from;
                    let d = transform_maps[k].from + transform_maps[k].len - 1;
                    if check_again.len() > 0 {
                        a = check_again[0].0;
                        b = check_again[0].1;
                        check_again.clear();
                        // println!("Check again");
                        // println!("i, j, k: {}, {}, {}", i, j, k);
                    }
                    // does this range intersect this map?
                    if b < c {
                        // range: 1-3       A--B
                        // map: 5-7              C--D
                        // no collision
                        // I can ignore it
                        next_ranges.push(ranges[j]);
                        break;
                    } else if a > d {
                        // range 6-10           A--B
                        // map: 1-4       C--D
                        // no collision
                        // try again with the next transform_map
                        if k == transform_maps.len() - 1 {
                            next_ranges.push(ranges[j]);
                            shortcut_index = k;
                        } else {
                            shortcut_index = k + 1;
                        }
                    } else {
                        // some kind of collision

                        // range: 30-40        A--B
                        // map: 10-60         C-----D
                        if a >= c && b <= d {
                            // 98 + 50 - 98 = 50
                            // 99 + 50 - 98 = 51
                            let new_a = a + transform_maps[k].to - transform_maps[k].from;
                            let new_b = b + transform_maps[k].to - transform_maps[k].from;
                            if new_a > new_b {
                                println!("broken 1 a: {}, b: {}, c: {}, d: {}", a, b, c, d);
                                println!("i, j, k: {}, {}, {}", i, j, k);
                            }
                            next_ranges.push((new_a, new_b));
                            shortcut_index = k;
                            break;
                        }
                        // range: 20-60        A------B
                        // map: 30-40            C--D
                        else if a <= c && b >= d {
                            // push a-c, c-d, but not d-b. It might match another map
                            if a > c {
                                println!("broken 1.5 a: {}, b: {}, c: {}, d: {}", a, b, c, d);
                                println!("i, j, k: {}, {}, {}", i, j, k);
                            }
                            // is this the only case where they can be exact?
                            // or just the only case in my input?
                            if a != c {
                                next_ranges.push((a, c - 1));
                            }

                            let new_c = transform_maps[k].to;
                            let new_d = transform_maps[k].to + transform_maps[k].len - 1;
                            if new_c > new_d {
                                println!("broken 2");
                            }
                            next_ranges.push((new_c, new_d));
                            //  I don't think I can just add to ranges
                            // while I'm already looping 0..ranges.len()
                            check_again.push((d + 1, b));
                            // don't break
                            // continue to the next transform_map
                            // for collisions with check_again
                            // it has to be the next one or none of them.
                        }
                        // range: 1-10         A--B
                        // map: 5-20             C--D
                        else if a <= c && b >= c {
                            if a > c-1 {
                                println!("broken 2.5 a: {}, b: {}, c: {}, d: {}", a, b, c, d);
                                println!("i, j, k: {}, {}, {}", i, j, k);
                            }
                            next_ranges.push((a, c - 1));

                            let new_c = transform_maps[k].to;
                            let new_b = b + transform_maps[k].to - transform_maps[k].from;
                            if new_c > new_b {
                                println!("broken 3");
                            }
                            next_ranges.push((new_c, new_b));
                            break;
                        }
                        // range: 10-20         A--B
                        // map: 5-15          C--D
                        else if a >= c && b >= d {
                            let new_a = a + transform_maps[k].to - transform_maps[k].from;
                            let new_d = transform_maps[k].to + transform_maps[k].len - 1;
                            if new_a > new_d {
                                println!("broken 4");
                            }
                            next_ranges.push((new_a, new_d));
                            check_again.push((d + 1, b));
                        } else {
                            println!("I missed a case j:{}, k:{}", j, k);
                        }
                    }
                }
            }

            // I want to skip the next line
            // I know the next line after a blank will be
            // soil-to-fertilizer map:
            skip_line = i + 1;
            // clear the old maps
            transform_maps.clear();

            // println!("Ranges: {:?}", &ranges);
            // println!("Next ranges: {:?}", &next_ranges);

            ranges = next_ranges.clone();
            ranges.sort();
        } else if i == skip_line {
            // skip the "seel-to-soil map:" lines
            // they come after blank lines
            continue;
        } else {
            let triple: Vec<u64> = lines[i]
                .split(" ")
                .filter_map(|x| x.parse::<u64>().ok())
                .collect();
            let map = TransformMap {
                to: triple[0],
                from: triple[1],
                len: triple[2],
            };
            transform_maps.push(map);
        }
    }

    ranges.sort();
    ranges[0].0
}

#[cfg(test)]
mod tests {

    use super::*;

    const BASIC_INPUT_DAY_05: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part_1_works() {
        assert_eq!(35, part_1(&BASIC_INPUT_DAY_05));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(46, part_2(&BASIC_INPUT_DAY_05));
    }

    #[test]
    fn sort_test() {
        let mut seeds: Vec<u32> = vec![79, 14, 55, 13];
        seeds.sort();
        // sort does sort smallest things first
        // this does work how I wanted it to
        assert_eq!(vec![13, 14, 55, 79], seeds);
    }
}
