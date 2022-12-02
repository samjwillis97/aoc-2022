use std::{collections::HashMap, str::FromStr};

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|round| one_map_solve(round)).sum())
    // Some(input.lines().map(|round| score_round(round)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|round| two_map_solve(round)).sum())
    // Some(input.lines().map(|round| game_round(round)).sum())
}

// Map Solves are around 50us
fn one_map_solve(input: &str) -> u32 {
    match input {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0,
    }
}

fn two_map_solve(input: &str) -> u32 {
    match input {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0,
    }
}

// Logic Solves are around 200us
fn score_round(round: &str) -> u32 {
    let round: Vec<&str> = round.split(' ').collect();
    let played = Weapon::from_str(round[1]).unwrap();
    let opponent = Weapon::from_str(round[0]).unwrap();

    let base_score = played.to_score();
    if played > opponent {
        return 6 + base_score;
    } else if played == opponent {
        return 3 + base_score;
    }
    return base_score;
}

fn game_round(round: &str) -> u32 {
    let round: Vec<&str> = round.split(' ').collect();
    let outcome = round[1];
    let opponent = Weapon::from_str(round[0]).unwrap();

    match outcome {
        "X" => match opponent {
            Weapon::Rock => Weapon::Scissors.to_score(),
            Weapon::Paper => Weapon::Rock.to_score(),
            Weapon::Scissors => Weapon::Paper.to_score(),
        },
        "Y" => opponent.to_score() + 3,
        "Z" => {
            (match opponent {
                Weapon::Rock => Weapon::Paper.to_score(),
                Weapon::Paper => Weapon::Scissors.to_score(),
                Weapon::Scissors => Weapon::Rock.to_score(),
            } + 6)
        }
        _ => 0,
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Weapon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Weapon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == &Weapon::Rock && other == &Weapon::Scissors {
            return std::cmp::Ordering::Greater;
        } else if self == &Weapon::Paper && other == &Weapon::Rock {
            return std::cmp::Ordering::Greater;
        } else if self == &Weapon::Scissors && other == &Weapon::Paper {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Less;
    }
}

impl FromStr for Weapon {
    type Err = ();
    fn from_str(input: &str) -> Result<Weapon, Self::Err> {
        match input {
            "A" => Ok(Weapon::Rock),
            "X" => Ok(Weapon::Rock),
            "B" => Ok(Weapon::Paper),
            "Y" => Ok(Weapon::Paper),
            "C" => Ok(Weapon::Scissors),
            "Z" => Ok(Weapon::Scissors),
            _ => Err(()),
        }
    }
}

trait ToScore {
    fn to_score(&self) -> u32;
}

impl ToScore for Weapon {
    fn to_score(&self) -> u32 {
        match self {
            Weapon::Rock => 1,
            Weapon::Paper => 2,
            Weapon::Scissors => 3,
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
