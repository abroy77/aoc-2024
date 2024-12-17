use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
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
        Node::new(add_points(self.point, self.dir), self.dir, self.cost + 1)
    }
    fn rotate(&self, clockwise: bool) -> Self {
        if clockwise {
            Node::new(self.point, rotate_clockwise(&self.dir), self.cost + 1000)
        } else {
            Node::new(
                self.point,
                rotate_anticlockwise(&self.dir),
                self.cost + 1000,
            )
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
    let mut seen: HashSet<Node> = HashSet::from([start_node]);
    let mut counter = 0;

    while let Some(node) = heap.pop() {
        if counter > 100000 {
            dbg!("breaking free");
            break;
        }
        // check if the next node is reachable in front

        let next_point = add_points(node.point, node.dir);
        if next_point == end_point {
            // struck gold!!
            return node.cost + 1;
        }
        if in_bounds(&slices, &next_point)
            && grid[next_point.0 as usize][next_point.1 as usize] == '.'
        {
            // we can move forward let's goooooooooo
            let next_node = node.move_forward();
            if !seen.contains(&next_node) {
                heap.push(next_node);
                seen.insert(next_node);
            }
        }
        // add the rotations to the end of the system

        let next_node = node.rotate(true);
        if !seen.contains(&next_node) {
            heap.push(next_node);
            seen.insert(next_node);
        }
        let next_node = node.rotate(false);
        if !seen.contains(&next_node) {
            heap.push(next_node);
            seen.insert(next_node);
        }

        seen.insert(node);
        counter += 1;
    }

    0
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
    fn test_sample() {
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
        assert_eq!(7036, solve(grid));
    }
}
