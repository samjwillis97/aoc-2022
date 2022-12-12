use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut head_position: (i32, i32) = (0, 0);
    let mut tail_position: (i32, i32) = (0, 0);
    let mut tail_visited: HashSet<String> = HashSet::new();
    tail_visited.insert(serialize_position(tail_position));

    input.lines().for_each(|movement| {
        let mut split_movement = movement.split(" ");
        let direction = split_movement.next().unwrap();
        for _ in 0..split_movement
            .next()
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0)
        {
            move_in_direction(
                match direction {
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => panic!("Should not be here!"),
                },
                &mut tail_position,
                &mut head_position,
            );
            tail_visited.insert(serialize_position(tail_position));
        }
    });

    Some(tail_visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

// X Y
fn move_in_direction(dir: Direction, tail: &mut (i32, i32), head: &mut (i32, i32)) {
    match dir {
        Direction::Right => {
            head.1 = head.1 + 1;
            if (head.1 - tail.1).abs() > 1 {
                tail.1 = tail.1 + 1;

                let vertical_diff = head.0 - tail.0;
                if vertical_diff > 0 {
                    tail.0 = tail.0 + 1;
                } else if vertical_diff < 0 {
                    tail.0 = tail.0 - 1;
                }
            }
        }
        Direction::Left => {
            head.1 = head.1 - 1;
            if (head.1 - tail.1).abs() > 1 {
                tail.1 = tail.1 - 1;

                let vertical_diff = head.0 - tail.0;
                if vertical_diff > 0 {
                    tail.0 = tail.0 + 1;
                } else if vertical_diff < 0 {
                    tail.0 = tail.0 - 1;
                }
            }
        }
        Direction::Up => {
            head.0 = head.0 + 1;
            if (head.0 - tail.0).abs() > 1 {
                tail.0 = tail.0 + 1;

                let horizontal_diff = head.1 - tail.1;
                if horizontal_diff > 0 {
                    tail.1 = tail.1 + 1;
                } else if horizontal_diff < 0 {
                    tail.1 = tail.1 - 1;
                }
            }
        }
        Direction::Down => {
            head.0 = head.0 - 1;
            if (head.0 - tail.0).abs() > 1 {
                tail.0 = tail.0 - 1;

                let horizontal_diff = head.1 - tail.1;
                if horizontal_diff > 0 {
                    tail.1 = tail.1 + 1;
                } else if horizontal_diff < 0 {
                    tail.1 = tail.1 - 1;
                }
            }
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn serialize_position(pos: (i32, i32)) -> String {
    return pos.0.to_string() + "|" + &pos.1.to_string();
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
