use std::collections::{HashMap, HashSet};

use itertools::Itertools;

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

// 800 us
pub fn part_two(input: &str) -> Option<u32> {
    let mut group_map = HashMap::new();
    let mut group_keys: Vec<u32> = Vec::new();

    input.lines().enumerate().for_each(|(i, elf)| {
        let mut elf_map = HashSet::new();
        elf.chars().for_each(|item| {
            elf_map.insert(convert_char_to_priority(item));
        });
        group_map.insert(i, elf_map);
    });

    // Would be nice to find a better way of doing this
    for i in (0..input.lines().count()).step_by(3) {
        let mut union_set = HashSet::new();

        group_map
            .get(&i)
            .unwrap()
            .intersection(group_map.get(&(&i + 1)).unwrap())
            .for_each(|value| {
                union_set.insert(*value);
            });

        group_keys.push(
            *union_set
                .intersection(group_map.get(&(&i + 2)).unwrap())
                .next()
                .unwrap(),
        );
    }

    Some(group_keys.iter().sum())
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
        assert_eq!(part_two(&input), Some(70));
    }
}
