use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
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

fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let row: Vec<usize> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap_or(100) as usize)
            .collect();
        result.push(row);
    }
    for i in 0..result.len() - 1 {
        assert_eq!(result[i].len(), result[i + 1].len());
    }
    result
}

fn in_bounds<T>(grid: &[&[T]], p: &Point) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as isize && p.1 < grid[0].len() as isize
}

fn get_start_points(grid: &[&[usize]]) -> Vec<Point> {
    let mut start_points = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                start_points.push((i as isize, j as isize));
            }
        }
    }
    start_points
}

fn get_trailhead_score(grid: &[&[usize]], start_point: Point) -> usize {
    let mut score = 0;
    let mut queue = vec![start_point];
    while let Some(point) = queue.pop() {
        if grid[point.0 as usize][point.1 as usize] == 9 {
            score += 1;
            continue;
        }
        for dir in DIRECTIONS {
            // get next_point
            let next_point = (point.0 + dir.0, point.1 + dir.1);
            if in_bounds(grid, &next_point) {
                // check if value is 1 more than the current point value
                if grid[next_point.0 as usize][next_point.1 as usize]
                    == grid[point.0 as usize][point.1 as usize] + 1
                {
                    // great! add to the queue
                    queue.push(next_point);
                }
            }
        }
    }
    score
}

fn solve(grid: &[&[usize]]) -> usize {
    let start_points = get_start_points(grid);
    start_points
        .iter()
        .map(|sp| get_trailhead_score(grid, *sp))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample() {
        let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let grid = parse_grid(input);
        let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
        assert_eq!(81, solve(&slices));
    }
}
