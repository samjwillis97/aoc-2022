use std::{collections::HashSet, str::FromStr};

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

// Ten Knots!
//2880 is TOO HIGH
pub fn part_two(input: &str) -> Option<u32> {
    let mut tail_visited: HashSet<String> = HashSet::new();
    let mut rope: [(i32, i32); 10] = [(0, 0); 10];
    input.lines().for_each(|movement| {
        let mut split_movement = movement.split(" ");
        let direction = split_movement.next().unwrap().parse::<Direction>().unwrap();
        println!("Head moving: {:?}", direction);
        for _ in 0..split_movement
            .next()
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0)
        {
            move_head(&direction, &mut rope);
            for i in 1..10 {
                follow_knot(i, &mut rope);
                if i == 9 {
                    tail_visited.insert(serialize_position(*rope.get(i).unwrap()));
                }
                println!("{:?}", rope);
            }
            // println!("{:?}", rope);
            println!("");
        }
    });

    Some(tail_visited.len().try_into().unwrap())
}

// X Y
// Y X?
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

fn move_head(dir: &Direction, rope: &mut [(i32, i32)]) {
    let mut head = rope.get_mut(0).unwrap();
    match dir {
        Direction::Left => head.0 = head.0 - 1,
        Direction::Right => head.0 = head.0 + 1,
        Direction::Up => head.1 = head.1 + 1,
        Direction::Down => head.1 = head.1 - 1,
    }
}

// Vertical is Y
// Horizontal is X
//
fn follow_knot(index: usize, rope: &mut [(i32, i32)]) {
    if index < 1 || index > 9 {
        return;
    }
    let head = rope.get(index - 1).unwrap().clone();
    let tail = rope.get_mut(index).unwrap();

    let horizontal_displacement = head.0 - tail.0;
    let vertical_displacement = head.1 - tail.1;

    // Go Up
    if vertical_displacement.abs() > 1 {
        if vertical_displacement > 0 {
            // println!("Knot {} Going UP!!!", index);
            tail.1 = tail.1 + 1;
        } else {
            // println!("Knot {} Going DOWN!!!", index);
            tail.1 = tail.1 - 1;
        }
        // Go Across as well
        if horizontal_displacement.abs() > 0 {
            if horizontal_displacement > 0 {
                // println!("And Right");
                tail.0 = tail.0 + 1;
            } else {
                // println!("And Left");
                tail.0 = tail.0 - 1;
            }
        }
    } else if horizontal_displacement.abs() > 1 {
        // Go Across
        if horizontal_displacement > 0 {
            // println!("Knot {} Going RIGHT!!!", index);
            tail.0 = tail.0 + 1;
        } else {
            // println!("Knot {} Going LEFT!!!", index);
            tail.0 = tail.0 - 1;
        }
        // Go Up as well
        if vertical_displacement > 0 {
            if vertical_displacement > 0 {
                // println!("And Up");
                tail.1 = tail.1 + 1;
            } else {
                // println!("And Down");
                tail.1 = tail.1 - 1;
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
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
        assert_eq!(part_two(&input), Some(1));
    }
}
