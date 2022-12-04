use regex::Regex;

// Solves down to 1.4ms, lots of room to improve I think
// using the parse_line function was taking around 3.55ms
// Initial Regex solve takes 3ms
// Reading through solutions realised i missed the obvious one...
// don't need to create ranges at all.
// first non range/set solution 1.5ms
// second, iterating with lines rather than regex captures, 1.5 -> 1ms
// interestingly part2 solves in 540us

// Just realising now that I should try this with a regex instead lol
pub fn part_one(input: &str) -> Option<u32> {
    let mut counter: u32 = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input.lines().for_each(|line| {
        let capture = re.captures(line).unwrap();

        let first_value = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let second_value = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let third_value = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let fourth_value = capture.get(4).unwrap().as_str().parse::<u32>().unwrap();
        if (first_value >= third_value && second_value <= fourth_value)
            || (third_value >= first_value && fourth_value <= second_value)
        {
            counter = counter + 1;
        }
    });

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter: u32 = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    input.lines().for_each(|line| {
        let capture = re.captures(line).unwrap();

        let first_value = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let second_value = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let third_value = capture.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let fourth_value = capture.get(4).unwrap().as_str().parse::<u32>().unwrap();
        if second_value >= third_value && first_value <= fourth_value
            || fourth_value >= first_value && third_value <= second_value
        {
            counter = counter + 1;
        }
    });

    Some(counter)
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
