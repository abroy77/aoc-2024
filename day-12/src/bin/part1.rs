use std::{
    collections::{HashSet, VecDeque},
    env,
    fs::read_to_string,
    path::PathBuf,
    str::FromStr,
};

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

fn in_bounds<T>(grid: &[&[T]], p: &Point) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as isize && p.1 < grid[0].len() as isize
}
fn point_add(a: &Point, b: &Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn solve<T: PartialEq>(grid: &[&[T]]) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut patches = vec![];
    let mut seen = HashSet::new();
    for i in 0..num_rows {
        for j in 0..num_cols {
            if !seen.contains(&(i as isize, j as isize)) {
                let patch = flood_fill(grid, &seen, &(i as isize, j as isize));
                seen.extend(patch.iter());
                patches.push(patch);
            }
        }
    }

    // we have the patches. now we need to score them
    let mut score = 0;
    for patch in patches.iter() {
        let area = patch.len();
        // getting the perimeter is more involved
        let mut perimeter = 0;
        for p in patch {
            // see how many of its neighbors are in this object
            // or we could check the grid as well
            for dir in DIRECTIONS.iter() {
                let n = point_add(p, dir);
                if !patch.contains(&n) {
                    perimeter += 1;
                }
            }
        }

        score += area * perimeter;
    }
    score
}

fn flood_fill<T: PartialEq>(
    grid: &[&[T]],
    seen: &HashSet<Point>,
    start_point: &Point,
) -> HashSet<Point> {
    let value = &grid[start_point.0 as usize][start_point.1 as usize];
    let mut members = HashSet::from([*start_point]);
    let mut queue = VecDeque::from([*start_point]);
    while let Some(p) = queue.pop_front() {
        for dir in DIRECTIONS.iter() {
            let neighbor = point_add(&p, dir);
            if in_bounds(grid, &neighbor)
                && !seen.contains(&neighbor)
                && !members.contains(&neighbor)
                && *value == grid[neighbor.0 as usize][neighbor.1 as usize]
            {
                members.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    members
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample() {
        let input = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let grid = parse_grid(input);
        let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
        assert_eq!(1930, solve(&slices));
    }
}
