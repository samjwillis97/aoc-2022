use regex::Regex;
use std::collections::HashSet;

// Solves down to 1.4ms, lots of room to improve I think
// using the parse_line function was taking around 3.55ms
// Initial Regex solve takes 3ms

// Just realising now that I should try this with a regex instead lol
pub fn part_one(input: &str) -> Option<u32> {
    let mut first_set = HashSet::new();
    let mut second_set = HashSet::new();

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|capture| {
                first_set.clear();
                second_set.clear();
                (capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
                    ..capture.get(2).unwrap().as_str().parse::<u32>().unwrap())
                    .for_each(|section| {
                        first_set.insert(section);
                    });

                (capture.get(3).unwrap().as_str().parse::<u32>().unwrap()
                    ..capture.get(4).unwrap().as_str().parse::<u32>().unwrap())
                    .for_each(|section| {
                        second_set.insert(section);
                    });

                return first_set.is_superset(&second_set) || second_set.is_superset(&first_set);
            })
            .filter(|value| *value)
            .count()
            .try_into()
            .unwrap(),
    )

    // Some(
    //     input
    //         .lines()
    //         .map(|line| {
    //             let mut splitted = line.split(",");
    //             let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    //             let capture = re.captures(line);

    //             for cap in re.captures_iter(line) {
    //                 cap.get(1).map(|v| v.as_str().parse::<u32>());
    //             }
    //             return (splitted.next().unwrap(), splitted.next().unwrap());
    //         })
    //         .map(|pair_string| {
    //             let mut first_pair = pair_string.0.split("-");
    //             let mut second_pair = pair_string.1.split("-");
    //             return (
    //                 first_pair.next().unwrap().parse::<u32>().unwrap()
    //                     ..first_pair.next().unwrap().parse::<u32>().unwrap() + 1,
    //                 second_pair.next().unwrap().parse::<u32>().unwrap()
    //                     ..second_pair.next().unwrap().parse::<u32>().unwrap() + 1,
    //             );
    //         })
    //         .map(|ranges| {
    //             first_set.clear();
    //             second_set.clear();
    //             ranges.0.for_each(|value| {
    //                 first_set.insert(value);
    //             });
    //             ranges.1.for_each(|value| {
    //                 second_set.insert(value);
    //             });
    //             return first_set.is_superset(&second_set) || second_set.is_superset(&first_set);
    //         })
    //         .filter(|value| *value)
    //         .count()
    //         .try_into()
    //         .unwrap(),
    // )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut first_set = HashSet::new();
    let mut second_set = HashSet::new();

    Some(
        input
            .lines()
            .map(|line| {
                let mut splitted = line.split(",");
                return (splitted.next().unwrap(), splitted.next().unwrap());
            })
            .map(|pair_string| {
                let mut first_pair = pair_string.0.split("-");
                let mut second_pair = pair_string.1.split("-");
                return (
                    first_pair.next().unwrap().parse::<u32>().unwrap()
                        ..first_pair.next().unwrap().parse::<u32>().unwrap() + 1,
                    second_pair.next().unwrap().parse::<u32>().unwrap()
                        ..second_pair.next().unwrap().parse::<u32>().unwrap() + 1,
                );
            })
            .map(|ranges| {
                first_set.clear();
                second_set.clear();
                ranges.0.for_each(|value| {
                    first_set.insert(value);
                });
                ranges.1.for_each(|value| {
                    second_set.insert(value);
                });
                return first_set.intersection(&second_set).count() > 0;
            })
            .filter(|value| *value)
            .count()
            .try_into()
            .unwrap(),
    )
}

fn parse_line(input: &str) -> (HashSet<u32>, HashSet<u32>) {
    let mut first_set = HashSet::new();
    let mut second_set = HashSet::new();

    let mut output = input.split(",").flat_map(|sections| {
        return sections.split("-").map(|bounds| {
            return bounds.parse::<u32>().unwrap();
        });
    });

    // TODO: Look at using chunks or something for this processing
    let first_value = output.next().unwrap();
    let second_value = output.next().unwrap() + 1;
    let third_value = output.next().unwrap();
    let fourth_value = output.next().unwrap() + 1;

    (first_value..second_value).for_each(|v| {
        first_set.insert(v);
    });

    (third_value..fourth_value).for_each(|v| {
        second_set.insert(v);
    });

    return (first_set, second_set);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
