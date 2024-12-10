use std::collections::{HashMap, HashSet};
use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

use itertools::Itertools;

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
    let grid_rows = grid.len();
    let grid_cols = grid[0].len();
    let grid_size = (grid_rows, grid_cols);
    let data = make_hashmap(grid);
    let sol = solve(&data, grid_size);
    println!("Solution is {}", sol);

    Ok(())
}

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

fn solve(data: &DatMap, grid_size: (usize, usize)) -> usize {
    let mut antinodes = HashSet::new();
    for (_, points) in data.iter() {
        // we need to iterate through all the pairs in the set
        // if the num of elements in it is greater than 2
        if points.len() < 2 {
            continue;
        }
        // get the 2-combos of the points
        for combo in points.iter().combinations(2) {
            let diff = (combo[1].0 - combo[0].0, combo[1].1 - combo[0].1);
            // go in the dir away from diff
            let mut step = 0;
            let mut anti1 = (combo[0].0 - step * diff.0, combo[0].1 - step * diff.1);
            while anti1.0 >= 0
                && anti1.0 < grid_size.0 as isize
                && anti1.1 >= 0
                && anti1.1 < grid_size.1 as isize
            {
                // we're in the grid. add to antinodes
                antinodes.insert(anti1);
                step += 1;
                anti1 = (combo[0].0 - step * diff.0, combo[0].1 - step * diff.1);
            }

            let mut step = 0;
            let mut anti2 = (combo[1].0 + step * diff.0, combo[1].1 + step * diff.1);
            while anti2.0 >= 0
                && anti2.0 < grid_size.0 as isize
                && anti2.1 >= 0
                && anti2.1 < grid_size.1 as isize
            {
                // we're in the grid. add to antinodes
                antinodes.insert(anti2);
                step += 1;
                anti2 = (combo[1].0 + step * diff.0, combo[1].1 + step * diff.1);
            }
        }
    }

    antinodes.len()
}

type Point = (isize, isize);
type DatMap = HashMap<char, HashSet<Point>>;
fn make_hashmap(grid: Vec<Vec<char>>) -> DatMap {
    let mut hm: HashMap<char, HashSet<Point>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !grid[i][j].is_alphanumeric() {
                continue;
            }
            hm.entry(grid[i][j])
                .and_modify(|e| {
                    e.insert((i as isize, j as isize));
                })
                .or_insert(HashSet::from([(i as isize, j as isize)]));
        }
    }
    hm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";
        let grid = parse_grid(input);
        let grid_rows = grid.len();
        let grid_cols = grid[0].len();
        let grid_size = (grid_rows, grid_cols);
        let data = make_hashmap(grid);
        assert_eq!(34, solve(&data, grid_size));
    }
}
