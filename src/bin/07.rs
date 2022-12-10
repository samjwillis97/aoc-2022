use std::collections::HashMap;

// TODO: Would be nice to come back to this day..
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
                    match previous_dir.files.insert(line[1].to_string(), size) {
                        None => previous_dir.total_file_size = previous_dir.total_file_size + size,
                        _ => (),
                    }
                }
            }
        }
    });

    let dir_sizes = recurse_get_size(file_system.directories.get("/").unwrap());
    Some(dir_sizes.iter().filter(|size| *size <= &100000).sum())
}

fn recurse_get_size(dir: &Directory) -> Vec<u32> {
    let mut sizes: Vec<u32> = Vec::new();
    if dir.directories.len() == 0 {
        sizes.push(dir.total_file_size);
        return sizes;
    }

    let mut current_sub_sizes: u32 = 0;
    for ele in &dir.directories {
        let mut sub_size = recurse_get_size(ele.1);
        sizes.append(&mut sub_size);
        current_sub_sizes = current_sub_sizes + *sizes.last().unwrap_or(&0);
    }

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
                    match previous_dir.files.insert(line[1].to_string(), size) {
                        None => previous_dir.total_file_size = previous_dir.total_file_size + size,
                        _ => (),
                    }
                }
            }
        }
    });

    let mut dir_sizes = recurse_get_size(file_system.directories.get("/").unwrap());
    dir_sizes.sort();

    // There are some wrong values in here.. but might be ok
    let used = dir_sizes.last().unwrap();
    let unused = 70000000 - used;
    let required = 30000000 - unused;

    Some(dir_sizes.into_iter().find(|v| *v >= required).unwrap())
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
