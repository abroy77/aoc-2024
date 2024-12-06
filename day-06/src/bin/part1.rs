use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

fn main() -> std::io::Result<()> {
    // get the data filepath
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Filepath not provided");
    }
    let data_path = PathBuf::from_str(&args[1]).expect("Failed to convert input to filepath");

    assert!(data_path.exists(), "data path does not exist");
    let data = read_to_string(data_path).expect("could not read datapath");
    let grid = parse_grid(&data);
    let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
    let sol = solve(&slices);
    println!("Solution is {}", sol);

    Ok(())
}
type Point = (isize, isize);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        result.push(row);
    }
    for i in 0..result.len() - 1 {
        assert_eq!(result[i].len(), result[i + 1].len());
    }
    result
}

fn rotate_right(p: &Point) -> Point {
    (p.1, -p.0)
}

fn in_bounds<T>(grid: &[&[T]], p: &Point) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as isize && p.1 < grid[0].len() as isize
}

fn get_start_point(grid: &[&[char]]) -> Point {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                return (i as isize, j as isize);
            }
        }
    }
    panic!("bro where is the start point fr fr");
}

fn solve(grid: &[&[char]]) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut tracker = vec![vec![false; num_cols]; num_rows];
    let mut p = get_start_point(grid);
    let mut dir = (-1, 0);
    loop {
        tracker[p.0 as usize][p.1 as usize] = true;
        let mut next_pos = (p.0 + dir.0, p.1 + dir.1);
        if !in_bounds(grid, &next_pos) {
            break;
        }
        if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            dir = rotate_right(&dir);
            next_pos = (p.0 + dir.0, p.1 + dir.1);
        }
        p = next_pos;
    }
    // out of loop. now sum up the tracker
    tracker
        .iter()
        .flat_map(|v| v.iter().filter(|x| **x))
        .count()
}

#[cfg(test)]
mod tests {
    use std::char;

    use super::*;

    #[test]
    fn test_parse() {
        let input = r"ab
cd";
        let answer: Vec<Vec<char>> = vec!["ab".chars().collect(), "cd".chars().collect()];
        let result = parse_grid(input);
        assert_eq!(answer, result);
    }

    #[test]
    fn test_sample() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let grid = parse_grid(input);
        let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
        assert_eq!(41, solve(&slices));
    }
}
