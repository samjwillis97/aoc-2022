use std::collections::HashSet;

// with the let mut sets in the map, 360us
// by initing them outside and draining in the map 260us
// assigning the value within the compartments.1 for_each has negligible effect
pub fn part_one(input: &str) -> Option<u32> {
    let mut item_map = HashSet::new();
    let mut doubles = HashSet::new();

    Some(
        input
            .lines()
            .map(|rucksack| {
                let compartments = rucksack.split_at(rucksack.len() / 2);
                compartments.0.chars().for_each(|item| {
                    item_map.insert(convert_char_to_priority(item));
                });
                compartments.1.chars().for_each(|item| {
                    let value = convert_char_to_priority(item);
                    if item_map.contains(&value) {
                        doubles.insert(value);
                    }
                });

                item_map.drain();
                doubles.drain().sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn convert_char_to_priority(letter: char) -> u32 {
    if letter.is_uppercase() {
        return letter.to_digit(36).unwrap() + 17;
    }
    return letter.to_digit(36).unwrap() - 9;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
