use std::{collections::HashSet, env, fs::read_to_string, path::PathBuf, str::FromStr};

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
    let sol = solve(&grid);
    println!("Solution is {}", sol);

    Ok(())
}

type Point = (isize, isize);
type Direction = (isize, isize);
type Step = (Point, Direction);

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

fn in_bounds(grid: &[Vec<char>], p: &Point) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as isize && p.1 < grid[0].len() as isize
}

fn get_start_point(grid: &[Vec<char>]) -> Point {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                return (i as isize, j as isize);
            }
        }
    }
    panic!("bro where is the start point fr fr");
}

fn add_obstacle(grid: &[Vec<char>], p: &Point) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid.iter().map(|row| row.to_vec()).collect();
    new_grid[p.0 as usize][p.1 as usize] = '#';
    new_grid
}

fn solve(grid: &[Vec<char>]) -> usize {
    // first we need to find all the steps that we would traverse if we did not add any
    let prev_steps = vec![(get_start_point(grid), (-1, 0))];
    let (normal_steps, _) = find_steps(grid, prev_steps);
    let (start_point, _) = normal_steps[0];
    let mut obstacle_locs = Vec::new();
    let mut seen_points = HashSet::new();
    // we will simulate adding an obstacle in each position in the path of the guard
    // excluding his / her starting position
    // we will also only try each point once. hence the seen_points set.
    for i in 1..normal_steps.len() {
        //skipping start position because we can't place an obstacle there
        let (p, _) = normal_steps[i];
        // can't have obstacle on starting square. skip
        if start_point == p || seen_points.contains(&p) {
            continue;
        }
        let new_grid = add_obstacle(grid, &p);
        // we need to provide the previous steps before, but not including 'i'
        let (_, loops) = find_steps(&new_grid, normal_steps[0..i].to_vec());
        seen_points.insert(p);
        if loops {
            obstacle_locs.push(p);
        }
    }

    obstacle_locs.iter().collect::<HashSet<_>>().len()
}
fn find_steps(grid: &[Vec<char>], mut prev_steps: Vec<Step>) -> (Vec<Step>, bool) {
    // the returned bool is true if a loop was found. else it is false
    let (mut p, mut dir) = prev_steps[prev_steps.len() - 1];
    let mut seen = HashSet::new();
    for s in prev_steps.iter() {
        seen.insert(((s.0 .0, s.0 .1), (s.1 .0, s.1 .1)));
    }
    loop {
        let mut next_pos = (p.0 + dir.0, p.1 + dir.1);
        if !in_bounds(grid, &next_pos) {
            break;
        }
        while grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            dir = rotate_right(&dir);
            next_pos = (p.0 + dir.0, p.1 + dir.1);
        }
        p = next_pos;
        prev_steps.push((p, dir));
        // check if it's in seen
        if seen.contains(&(p, dir)) {
            // found a loop!
            return (prev_steps, true);
        }
        seen.insert(prev_steps[prev_steps.len() - 1]);
        // add to prev and then add to hashset
    }
    (prev_steps, false)
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
        assert_eq!(6, solve(&grid));
    }
}
