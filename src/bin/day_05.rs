use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day_05_input.txt").unwrap();

    println!("{}", part_1(&input));
    // 1881211815 too high
    // whoops forgot to sort before output
    // 403695602 correct
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
                        shortcut_index = k+1;
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
    fn sort_test() {
        let mut seeds: Vec<u32> = vec![79, 14, 55, 13];
        seeds.sort();
        // sort does sort smallest things first
        // this does work how I wanted it to
        assert_eq!(vec![13, 14, 55, 79], seeds);
    }
}
