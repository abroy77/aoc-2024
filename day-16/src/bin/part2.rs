use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fs::read_to_string,
    hash::Hash,
    path::PathBuf,
    str::FromStr,
};

type Point = (isize, isize);
#[derive(Debug, Clone, Copy)]
struct Node {
    point: Point,
    dir: Point,
    cost: usize,
}

impl Node {
    fn new(point: Point, dir: Point, cost: usize) -> Self {
        Node { point, dir, cost }
    }
    fn move_forward(&self) -> Self {
        let next_point = add_points(self.point, self.dir);
        Node::new(next_point, self.dir, self.cost + 1)
    }
    fn rotate(&self, clockwise: bool) -> Self {
        if clockwise {
            let next_dir = rotate_clockwise(&self.dir);
            let next_point = add_points(self.point, next_dir);
            Node::new(next_point, next_dir, self.cost + 1001)
        } else {
            let next_dir = rotate_anticlockwise(&self.dir);
            let next_point = add_points(self.point, next_dir);
            Node::new(next_point, next_dir, self.cost + 1001)
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.dir == other.dir
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state);
        self.dir.hash(state);
    }
}

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
    let result = solve(grid);

    println!("Solution is {}", result);

    Ok(())
}

fn rotate_clockwise(p: &Point) -> Point {
    (p.1, -p.0)
}

fn rotate_anticlockwise(p: &Point) -> Point {
    (-p.1, p.0)
}

fn get_neighbors(grid: &[&[char]], node: Node) -> Vec<Node> {
    // returns the neighbor nodes
    [node.move_forward(), node.rotate(true), node.rotate(false)]
        .into_iter()
        .filter(|neighbor| {
            in_bounds(grid, &neighbor.point)
                && grid[neighbor.point.0 as usize][neighbor.point.1 as usize] != '#'
            //         neighbors.push(neighbor)
        })
        .collect()
}

fn solve(grid: Vec<Vec<char>>) -> usize {
    let slices: Vec<&[char]> = grid.iter().map(|v| v.as_slice()).collect();
    // okay we are doing a BFS where the cost of moving forward is 1,
    // cost of rotating by 90 degrees clockwise / anticlockwise is 1000
    // so normally we'll add the turns to the far end of the queue to be explored last
    // this is possible because the grid is smaller than 1000 in both directions so the cost of a turn
    // is always greater than the net cost of the longest sequence of forward moves
    let start_point = find_point(&slices, 'S');
    let end_point = find_point(&slices, 'E');
    let dir = (0, 1);
    let start_node = Node::new(start_point, dir, 0);
    let mut heap = BinaryHeap::from([start_node]);
    // make a visited map which tracks all the traversed paths. key is a point,
    // value is a hashset of points that are parents to the key
    let mut visited: HashMap<Point, HashSet<Point>> = HashMap::new();
    let mut shortest_pash_cost = None;
    let mut counter = 0;
    let mut seen: HashSet<Node> = HashSet::new();

    while let Some(node) = heap.pop() {
        // if counter < 10 {
        //     dbg!(counter);
        //     dbg!(node.point);
        //     dbg!(node.dir);
        // }
        if counter > 1000000 {
            dbg!("breaking free");
            dbg!(visited.len());
            break;
        }
        if let Some(shortest_path_cost) = shortest_pash_cost {
            if node.cost > shortest_path_cost {
                dbg!("longer than shortest path");
                break;
            }
        }

        if node.point == end_point {
            dbg!("found best path");
            shortest_pash_cost = Some(node.cost);
        }

        for neighbor in get_neighbors(&slices, node).iter() {
            if !seen.contains(neighbor) {
                heap.push(*neighbor);
                seen.insert(*neighbor);
                visited
                    .entry(neighbor.point)
                    .and_modify(|set| {
                        set.insert(node.point);
                    })
                    .or_insert_with(|| HashSet::from([node.point]));
            }
        }

        seen.insert(node);

        counter += 1;
    }

    // searching is complete. now we need to backtrack on visited map
    // to find all the shortest paths

    dbg!(shortest_pash_cost);
    // return 0;
    let all_paths = reconstruct_all_paths(start_point, end_point, &visited);

    all_paths
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

fn reconstruct_all_paths(
    start_point: Point,
    end_point: Point,
    visited: &HashMap<Point, HashSet<Point>>,
) -> Vec<Vec<Point>> {
    let mut all_paths = Vec::new();
    backtrack(
        end_point,
        start_point,
        visited,
        &mut Vec::new(),
        &mut all_paths,
    );
    all_paths
}

fn backtrack(
    current_point: Point,
    start_point: Point,
    visited: &HashMap<Point, HashSet<Point>>,
    path: &mut Vec<Point>,
    all_paths: &mut Vec<Vec<Point>>,
) {
    path.push(current_point);

    if current_point == start_point {
        all_paths.push(path.iter().rev().cloned().collect());
    } else {
        for parent in visited[&current_point].iter() {
            // recursively call backtrack from them
            dbg!(current_point);
            dbg!(parent);
            backtrack(*parent, start_point, visited, path, all_paths);
        }
    }
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

fn find_point<T: PartialEq>(grid: &[&[T]], element: T) -> Point {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == element {
                return (i as isize, j as isize);
            }
        }
    }
    panic!("bro where is the start point fr fr");
}

fn in_bounds<T>(grid: &[&[T]], p: &Point) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < grid.len() as isize && p.1 < grid[0].len() as isize
}

fn add_points(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
} // fn solve(

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let grid = parse_grid(input);
        assert_eq!(45, solve(grid));
    }

    #[test]
    fn test_sample_2() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let grid = parse_grid(input);
        assert_eq!(64, solve(grid));
    }
}
