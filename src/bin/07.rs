use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut file_system = Directory {
        files: HashMap::new(),
        directories: HashMap::new(),
        total_file_size: 0,
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
                                total_file_size: 0,
                            },
                        );
                    }
                    previous_dir = previous_dir.directories.get_mut(dir).unwrap();
                }

                if line[0] != "dir" {
                    let size = line[0].parse::<u32>().unwrap();
                    previous_dir.files.insert(line[1].to_string(), size);
                    previous_dir.total_file_size = previous_dir.total_file_size + size;
                }
            }
        }
    });

    println!("{:?}", file_system.directories.get("/").unwrap());
    let dir_sizes = recurse_get_size(file_system.directories.get("/").unwrap());
    println!("{:?}", dir_sizes);
    // Some(dir_sizes.iter().filter(|size| *size <= &100000).sum())
    None
}

fn recurse_get_size(dir: &Directory) -> Vec<u32> {
    let mut sizes: Vec<u32> = Vec::new();
    if dir.directories.len() == 0 {
        sizes.push(dir.total_file_size);
        println!("no sub: {}", dir.total_file_size);
        return sizes;
    }

    for ele in dir.directories.values() {
        sizes.append(&mut recurse_get_size(ele));
    }

    let current_sub_sizes: u32 = sizes.iter().sum();
    println!("sub: {}", current_sub_sizes);
    println!("current: {}", dir.total_file_size);
    println!("sum: {}", dir.total_file_size + current_sub_sizes);
    sizes.push(dir.total_file_size + current_sub_sizes);

    return sizes;
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut file_system = Directory {
        files: HashMap::new(),
        directories: HashMap::new(),
        total_file_size: 0,
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
                                total_file_size: 0,
                            },
                        );
                    }
                    previous_dir = previous_dir.directories.get_mut(dir).unwrap();
                }

                if line[0] != "dir" {
                    let size = line[0].parse::<u32>().unwrap();
                    previous_dir.files.insert(line[1].to_string(), size);
                    previous_dir.total_file_size = previous_dir.total_file_size + size;
                }
            }
        }
    });

    let mut dir_sizes = recurse_get_size(file_system.directories.get("/").unwrap());
    dir_sizes.sort();

    println!("{:?}", file_system.directories.get("/").unwrap());

    println!("{:?}", dir_sizes);

    // There are some wrong values in here.. but might be ok
    let used = dir_sizes.last().unwrap();
    println!("{:?}", used);
    let unused = 70000000 - used;
    println!("{:?}", unused);
    let required = 30000000 - unused;
    println!("{:?}", required);
    None
}

#[derive(Debug)]
struct Directory {
    files: HashMap<String, u32>,
    directories: HashMap<String, Directory>,
    total_file_size: u32,
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
        assert_eq!(part_two(&input), Some(24933642));
    }
}
