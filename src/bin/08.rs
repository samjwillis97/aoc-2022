// HOW DO I BUILD A GRID USING THIS :(
struct Grid<T> {
    value: T,
    left: Option<Box<Grid<T>>>,
    top: Option<Box<Grid<T>>>,
    right: Option<Box<Grid<T>>>,
    bottom: Option<Box<Grid<T>>>,
}

impl<T> Grid<T> {
    fn new(value: T) -> Self {
        Grid {
            value,
            left: None,
            top: None,
            right: None,
            bottom: None,
        }
    }
}

// TODO: Should keep track of everywhere that is clear really...
fn check_left(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> bool {
    if element.1 == 0 {
        return true;
    }

    let next_element = (element.0, element.1 - 1);

    if height > grid[next_element.0][next_element.1] {
        return check_left(grid, height, next_element);
    }

    return false;
}

fn check_right(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> bool {
    if element.1 == grid[0].len() - 1 {
        return true;
    }

    let next_element = (element.0, element.1 + 1);

    if height > grid[next_element.0][next_element.1] {
        return check_right(grid, height, next_element);
    }

    return false;
}

fn check_up(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> bool {
    if element.0 == 0 {
        return true;
    }

    let next_element = (element.0 - 1, element.1);

    if height > grid[next_element.0][next_element.1] {
        return check_up(grid, height, next_element);
    }

    return false;
}

fn check_down(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> bool {
    if element.0 == grid.len() - 1 {
        return true;
    }

    let next_element = (element.0 + 1, element.1);

    if height > grid[next_element.0][next_element.1] {
        return check_down(grid, height, next_element);
    }

    return false;
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|row| {
            row.split("")
                .filter(|v| *v != "")
                .map(|tree| tree.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut visible = 0;
    for i in 0..grid.len() {
        for j in 0..grid.get(i).unwrap().len() {
            let height = grid.get(i).unwrap().get(j).unwrap();
            if check_left(&grid, height.to_owned(), (i, j))
                || check_right(&grid, height.to_owned(), (i, j))
                || check_up(&grid, height.to_owned(), (i, j))
                || check_down(&grid, height.to_owned(), (i, j))
            {
                visible = visible + 1;
            }
        }
    }

    Some(visible)
}

fn view_left(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> u32 {
    let mut count = 0;

    if element.1 == 0 {
        return count;
    }

    let next_element = (element.0, element.1 - 1);

    count = count + 1;
    if height > grid[next_element.0][next_element.1] {
        count = count + view_left(grid, height, next_element);
    }

    return count;
}

fn view_right(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> u32 {
    let mut count = 0;

    if element.1 == grid[0].len() - 1 {
        return count;
    }

    let next_element = (element.0, element.1 + 1);

    count = count + 1;
    if height > grid[next_element.0][next_element.1] {
        count = count + view_right(grid, height, next_element);
    }

    return count;
}

fn view_up(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> u32 {
    let mut count = 0;
    if element.0 == 0 {
        return count;
    }

    count = count + 1;
    let next_element = (element.0 - 1, element.1);

    if height > grid[next_element.0][next_element.1] {
        count = count + view_up(grid, height, next_element);
    }

    return count;
}

fn view_down(grid: &Vec<Vec<u32>>, height: u32, element: (usize, usize)) -> u32 {
    let mut count = 0;

    if element.0 == grid.len() - 1 {
        return count;
    }

    count = count + 1;
    let next_element = (element.0 + 1, element.1);

    if height > grid[next_element.0][next_element.1] {
        count = count + view_down(grid, height, next_element);
    }

    return count;
}

// TODO: Could skip the edge because they are always going to be zero
pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|row| {
            row.split("")
                .filter(|v| *v != "")
                .map(|tree| tree.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut scenic_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid.get(i).unwrap().len() {
            println!("i: {}, j: {}", i, j);
            let height = grid.get(i).unwrap().get(j).unwrap();
            let element = (i, j);
            let score = view_left(&grid, *height, element)
                * view_up(&grid, *height, element)
                * view_right(&grid, *height, element)
                * view_down(&grid, *height, element);
            println!("left: {}", view_left(&grid, *height, element));
            println!("right: {}", view_right(&grid, *height, element));
            println!("up: {}", view_up(&grid, *height, element));
            println!("down: {}", view_down(&grid, *height, element));
            println!("score: {}", &score);
            println!("");
            if score > scenic_score {
                scenic_score = score;
            }
        }
    }

    Some(scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
