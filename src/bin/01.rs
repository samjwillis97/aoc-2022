pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|set| {
                set.lines()
                    .map(|line| line.parse::<u32>().unwrap_or(0))
                    .reduce(|accum, line| accum + line)
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut array: [u32; 3] = [0, 0, 0];
    input.split("\n\n").for_each(|set| {
        match set
            .lines()
            .map(|line| line.parse::<u32>().unwrap_or(0))
            .reduce(|accum, line| accum + line)
        {
            Some(num) if num > array[0] => {
                array[2] = array[1];
                array[1] = array[0];
                array[0] = num;
            }
            Some(num) if num > array[1] => {
                array[2] = array[1];
                array[1] = num;
            }
            Some(num) if num > array[2] => {
                array[2] = num;
            }
            _ => (),
        }
    });

    Some(array[0] + array[1] + array[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
