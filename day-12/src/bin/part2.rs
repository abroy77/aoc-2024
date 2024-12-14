use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    env,
    fmt::Debug,
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
// same actual types. aliasing for readability
type Dir = Point;

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

fn solve<T: PartialEq + Debug>(grid: &[&[T]]) -> usize {
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

    // we have the patches. now for each patch we need to calculate the number of sides it has

    let mut score = 0;
    for patch in patches.iter() {
        let area = patch.len();
        // getting the num of sides is more involved
        let num_sides = count_sides(grid, patch);

        score += area * num_sides;
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

fn get_point_edges<T: PartialEq>(grid: &[&[T]], p: Point) -> HashSet<Dir> {
    let mut edges = HashSet::new();
    let value = &grid[p.0 as usize][p.1 as usize];
    for dir in DIRECTIONS.iter() {
        let n = point_add(&p, dir);
        if in_bounds(grid, &n) && *value != grid[n.0 as usize][n.1 as usize] {
            // don't matche so is an edge
            edges.insert(*dir);
        }

        // or if it's out of bounds it's a boundary
        if !in_bounds(grid, &n) {
            edges.insert(*dir);
        }
    }
    edges
}

type PeMap = HashMap<Point, HashSet<Dir>>;

fn flood_fill_point_edges<T: PartialEq>(
    grid: &[&[T]],
    pe_map: &PeMap,
    p: Point,
    e: Dir,
) -> HashSet<(Point, Dir)> {
    let directions = if e == (-1, 0) || e == (1, 0) {
        // if edge pointing up or down. explore left right
        [(0, -1), (0, 1)]
    } else if e == (0, 1) || e == (0, -1) {
        // if edge pointing left right explore up down
        [(-1, 0), (1, 0)]
    } else {
        panic!("unknown direction")
    };

    let mut seen_points = HashSet::from([p]);
    let mut queue = VecDeque::from([p]);
    let mut point_edges: HashSet<(Point, Dir)> = HashSet::from([(p, e)]);

    while let Some(p) = queue.pop_front() {
        for dir in directions.iter() {
            let neighbor = point_add(&p, dir);
            if in_bounds(grid, &neighbor) && !seen_points.contains(&neighbor) {
                // not explored this point before.
                // get the edges of the neighbor
                if let Some(neighbor_edges) = pe_map.get(&neighbor) {
                    // if the neighbor is in the pe_map we can now look for the set of it's edges
                    if neighbor_edges.contains(&e) {
                        // aha we share an edge.
                        // add this to point_edges
                        point_edges.insert((neighbor, e));
                        queue.push_back(neighbor);
                    }
                }
                seen_points.insert(neighbor);
            }
        }
    }
    point_edges
}

fn get_patch_point_edge_map<T: PartialEq>(grid: &[&[T]], patch: &HashSet<Point>) -> PeMap {
    // returns a set of tuples.
    // first element is the coordinate of the element with the edge.
    // second element is the direction (up down left right) that the edge is facing
    let mut point_edges: HashMap<(isize, isize), HashSet<(isize, isize)>> = HashMap::new();
    for p in patch.iter() {
        let edges = get_point_edges(grid, *p);
        point_edges.insert(*p, edges);
    }

    point_edges
}

fn count_sides<T: PartialEq>(grid: &[&[T]], patch: &HashSet<Point>) -> usize {
    let mut count = 0;
    // get all the point edges
    let point_edges_map = get_patch_point_edge_map(grid, patch);

    let mut seen: HashSet<(Point, Dir)> = HashSet::new();
    for (p, edges) in point_edges_map.iter() {
        for edge in edges.iter() {
            if seen.contains(&(*p, *edge)) {
                continue;
            }

            // found a point edge we have not seen before
            count += 1;
            // find all contiguous point edges
            for pe in flood_fill_point_edges(grid, &point_edges_map, *p, *edge).iter() {
                seen.insert(*pe);
            }
            //insert this pe into seen itself
            seen.insert((*p, *edge));
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sample_3() {
        let input = r"AAAA";

        let grid = parse_grid(input);
        let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
        assert_eq!(16, solve(&slices));
    }
    #[test]
    fn test_sample_1() {
        let input = r"AAAA
BBCD
BBCC
EEEC";

        let grid = parse_grid(input);
        let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
        assert_eq!(80, solve(&slices));
    }
    #[test]
    fn test_sample_2() {
        let input = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        let grid = parse_grid(input);
        let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
        assert_eq!(236, solve(&slices));
    }
    #[test]
    fn test_sample_4() {
        let input = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        let grid = parse_grid(input);
        let slices: Vec<_> = grid.iter().map(|v| v.as_slice()).collect();
        assert_eq!(368, solve(&slices));
    }
}
