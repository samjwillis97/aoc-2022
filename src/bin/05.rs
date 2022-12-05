use std::collections::HashMap;

use itertools::Itertools;

// 190us
pub fn part_one(input: &str) -> Option<String> {
    let mut iter = input.split("\n\n");
    let stacks = iter.next().unwrap();
    let commands = iter.next().unwrap();

    let mut warehouse: HashMap<usize, Vec<&str>> = HashMap::new();

    let binding = stacks.split("\n").collect::<Vec<&str>>();
    let stack_iter = binding.iter().rev();

    stack_iter.skip(1).for_each(|crate_line| {
        crate_line
            .split("")
            .skip(2)
            .step_by(4)
            .enumerate()
            .for_each(|(i, value)| {
                let i = i + 1;
                if value == " ".to_string() {
                    return;
                }

                if warehouse.contains_key(&i) {
                    warehouse.get_mut(&i).unwrap().push(value)
                } else {
                    let mut stack: Vec<&str> = Vec::new();
                    stack.push(value);
                    warehouse.insert(i, stack);
                }
            })
    });

    commands.lines().for_each(|command| {
        let mut split = command.split(" ").into_iter();

        let iterations = split.nth(1).unwrap().parse::<u32>().unwrap();
        let move_from = split.nth(1).unwrap().parse::<u32>().unwrap();
        let to_move_to = &split.nth(1).unwrap().parse::<usize>().unwrap();

        for _ in 0..iterations {
            let to_move = warehouse
                .get_mut(&move_from.try_into().unwrap())
                .unwrap()
                .pop()
                .unwrap();

            if warehouse.contains_key(to_move_to) {
                warehouse.get_mut(to_move_to).unwrap().push(to_move);
            } else {
                let mut stack: Vec<&str> = Vec::new();
                stack.push(to_move);
                warehouse.insert(*to_move_to, stack);
            }
        }
    });

    Some(
        warehouse
            .into_iter()
            .sorted_by_key(|x| x.0)
            .map(|mut v| v.1.pop().unwrap())
            .join(""),
    )
}

// 165us
pub fn part_two(input: &str) -> Option<String> {
    let mut iter = input.split("\n\n");
    let stacks = iter.next().unwrap();
    let commands = iter.next().unwrap();

    let mut warehouse: HashMap<usize, Vec<&str>> = HashMap::new();

    let binding = stacks.split("\n").collect::<Vec<&str>>();
    let stack_iter = binding.iter().rev();

    stack_iter.skip(1).for_each(|crate_line| {
        crate_line
            .split("")
            .skip(2)
            .step_by(4)
            .enumerate()
            .for_each(|(i, value)| {
                let i = i + 1;
                if value == " ".to_string() {
                    return;
                }

                if warehouse.contains_key(&i) {
                    warehouse.get_mut(&i).unwrap().push(value)
                } else {
                    let mut stack: Vec<&str> = Vec::new();
                    stack.push(value);
                    warehouse.insert(i, stack);
                }
            })
    });

    commands.lines().for_each(|command| {
        let mut split = command.split(" ").into_iter();

        let move_count = split.nth(1).unwrap().parse::<u32>().unwrap();
        let move_from = split.nth(1).unwrap().parse::<u32>().unwrap();
        let to_move_to = split.nth(1).unwrap().parse::<usize>().unwrap();

        let moving_stack_length = warehouse.get(&move_from.try_into().unwrap()).unwrap().len();
        let mut to_move = warehouse
            .get_mut(&move_from.try_into().unwrap())
            .unwrap()
            .split_off(
                moving_stack_length - (<u32 as TryInto<usize>>::try_into(move_count).unwrap()),
            );

        if warehouse.contains_key(&to_move_to) {
            warehouse.get_mut(&to_move_to).unwrap().append(&mut to_move);
        } else {
            warehouse.insert(to_move_to, to_move);
        }
    });

    Some(
        warehouse
            .into_iter()
            .sorted_by_key(|x| x.0)
            .map(|mut v| v.1.pop().unwrap())
            .join(""),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
