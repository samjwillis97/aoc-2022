use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut seen_set: HashSet<&str> = HashSet::new();
    let binding = input
        .trim()
        .split("")
        .skip(1) // Seems to be whitespace at front.. and back
        .collect::<Vec<&str>>();
    let windows = binding.as_slice().windows(4);

    let mut i = 0;
    for window in windows {
        let window = window.to_vec();
        seen_set.clear();

        for ele in &window {
            seen_set.insert(ele);
        }

        if seen_set.len() == 4 {
            break;
        }
        i = i + 1;
    }

    Some(i + 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut seen_set: HashSet<&str> = HashSet::new();
    let binding = input
        .trim()
        .split("")
        .skip(1) // Seems to be whitespace at front.. and back
        .collect::<Vec<&str>>();
    let windows = binding.as_slice().windows(14);

    let mut i = 0;
    for window in windows {
        let window = window.to_vec();
        seen_set.clear();

        for ele in &window {
            seen_set.insert(ele);
        }

        if seen_set.len() == 14 {
            break;
        }
        i = i + 1;
    }

    Some(i + 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
