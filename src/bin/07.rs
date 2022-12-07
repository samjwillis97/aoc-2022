use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut file_system = Directory {
        files: HashMap::new(),
        directories: HashMap::new(),
    };

    let mut current_location: Vec<String> = Vec::new();

    input.lines().for_each(|line| {
        let line: Vec<&str> = line.split(" ").collect();
        match line[0] {
            "$" => match line[1] {
                "cd" => match line[2] {
                    ".." => {
                        current_location.pop();
                    }
                    v => {
                        current_location.push(v.to_string());
                    }
                },
                _ => (),
            },
            _ => {
                // I hate this but seems to create the directories for me..
                let mut previous_dir = &mut file_system;
                for dir in &current_location {
                    if !previous_dir.directories.contains_key(dir) {
                        previous_dir.directories.insert(
                            dir.to_string(),
                            Directory {
                                files: HashMap::new(),
                                directories: HashMap::new(),
                            },
                        );
                    }
                    previous_dir = previous_dir.directories.get_mut(dir).unwrap();
                }

                if line[0] != "dir" {
                    previous_dir
                        .files
                        .insert(line[1].to_string(), line[0].parse::<u32>().unwrap());
                }
            }
        }
    });

    let dir_sizes = recurse_get_size(file_system.directories.get("/").unwrap());
    Some(dir_sizes.iter().filter(|size| *size <= &100000).sum())
}

fn recurse_get_size(dir: &Directory) -> Vec<u32> {
    let mut sizes: Vec<u32> = Vec::new();
    let file_sizes = dir.files.values().sum();
    if dir.directories.len() == 0 {
        sizes.push(file_sizes);
        return sizes;
    }

    for ele in dir.directories.values() {
        sizes.append(&mut recurse_get_size(ele));
    }

    let current_sub_sizes: u32 = sizes.iter().sum();
    sizes.push(file_sizes + current_sub_sizes);

    return sizes;
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn handle_command(input: &str) {}

#[derive(Debug)]
struct Directory {
    files: HashMap<String, u32>,
    directories: HashMap<String, Directory>,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
