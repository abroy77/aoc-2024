use core::panic;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    fmt::{Debug, Display},
    fs::read_to_string,
    hash::Hash,
    path::PathBuf,
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

type Point = (isize, isize);
#[derive(Debug, Clone, Copy)]
struct Node {
    point: Point,
    cost: usize,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "point: ({}, {}),  cost {}",
            self.point.0, self.point.1, self.cost
        )
    }
}

impl Node {
    fn new(point: Point, cost: usize) -> Self {
        Node { point, cost }
    }

    fn get_neighbors(&self, grid: &[Vec<char>]) -> Vec<Node> {
        DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let new_point = add_points(self.point, *dir);
                if in_bounds(grid, &new_point)
                    && grid[new_point.0 as usize][new_point.1 as usize] != '#'
                {
                    Some(new_point)
                } else {
                    None
                }
            })
            .map(|np| Node::new(np, self.cost + 1))
            .collect()
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
        self.point == other.point
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

struct Visited {
    sources: HashMap<Node, HashSet<Node>>,
    costs: HashMap<Node, usize>,
}

impl Visited {
    fn new() -> Self {
        Visited {
            sources: HashMap::new(),
            costs: HashMap::new(),
        }
    }
    fn insert(&mut self, source: Node, dest: Node, cost: usize) -> bool {
        // return false if the insert was not made. else enter true
        // we assume that both sources and costs should have the same existing keys at any given
        // point in time
        if let Some(lowest_cost) = self.costs.get(&dest) {
            match cost.cmp(lowest_cost) {
                Ordering::Greater => {
                    return false;
                }
                Ordering::Less => {
                    // we found a better route to this dest. reset the sources
                    // and the cost
                    self.costs
                        .entry(dest)
                        .and_modify(|saved_cost| *saved_cost = cost);
                    assert_eq!(*self.costs.get(&dest).unwrap(), cost);
                    self.sources.entry(dest).and_modify(|inner_sources| {
                        println!("draining {}", &dest);
                        inner_sources.drain();
                        assert!(inner_sources.is_empty());
                        inner_sources.insert(source);
                    });
                }
                Ordering::Equal => {
                    // same as lowest cost. insert into sources
                    self.sources.entry(dest).and_modify(|e| {
                        e.insert(source);
                    });
                }
            }
        } else {
            // we need to make new entries in sources and in costs
            self.costs.insert(dest, cost);
            self.sources.insert(dest, HashSet::from([source]));
        }
        true
    }

    fn get_shortest_paths(&mut self, start_point: Point, end_point: Point) -> HashSet<Vec<Node>> {
        // we need to construct 4 dummy end_nodes such that the end_point can be reached from any direction
        let end_node = Node::new(end_point, 10000000);

        let mut shortest_paths = HashSet::new();
        self.backtrack(end_node, start_point, &mut shortest_paths, &mut Vec::new());

        // shortest_paths
        //     .iter()
        //     .flatten()
        //     .map(|n| n.point)
        //     .collect::<HashSet<_>>()
        //     .len()
        shortest_paths
    }

    fn backtrack(
        &mut self,
        current: Node,
        source: Point,
        all_paths: &mut HashSet<Vec<Node>>,
        path: &mut Vec<Node>,
    ) {
        path.push(current);

        if current.point == source {
            all_paths.insert(path.clone().into_iter().rev().collect());
        } else {
            let parents = self
                .sources
                .get(&current)
                .unwrap_or_else(|| {
                    dbg!(&current);
                    panic!();
                })
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for parent in parents {
                if path.contains(&parent) {
                    continue;
                }
                let mut cloned_path = path.clone();
                self.backtrack(parent, source, all_paths, &mut cloned_path);
            }
        }
    }
    fn print(&self) -> String {
        let mut ans = String::new();
        for (k, v) in self.sources.iter() {
            ans.push_str(&format!("{}", k));
            ans.push_str(" : [");
            for s in v {
                ans.push_str(&format!("{}", s));
                ans.push_str(", ");
            }
            ans.push(']');
            ans.push('\n');
        }
        ans.push('\n');
        ans
    }
}

fn print_tuple_2<T: Display>(a: &(T, T)) -> String {
    format!("({},{})", a.0, a.1)
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
    let (_, walls) = parse_walls(&data).unwrap();
    let result = solve(&walls[0..1024], (70, 70));

    println!("Solution is {}", result);

    Ok(())
}

fn rotate_clockwise(p: &Point) -> Point {
    (p.1, -p.0)
}

fn rotate_anticlockwise(p: &Point) -> Point {
    (-p.1, p.0)
}

fn solve(walls: &[Point], grid_size: (isize, isize)) -> usize {
    let grid_points = walls.iter().cloned().collect::<HashSet<Point>>();
    let grid: Vec<Vec<char>> = (0..grid_size.0 + 1)
        .map(|i| {
            (0..grid_size.1 + 1)
                .map(|j| {
                    if grid_points.contains(&(i, j)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();

    // let slices: Vec<&[char]> = grid.iter().map(|v| v.as_slice()).collect();
    // okay we are doing a BFS where the cost of moving forward is 1,
    // cost of rotating by 90 degrees clockwise / anticlockwise is 1000
    // so normally we'll add the turns to the far end of the queue to be explored last
    // this is possible because the grid is smaller than 1000 in both directions so the cost of a turn
    // is always greater than the net cost of the longest sequence of forward moves
    let start_point = (0, 0);
    let end_point = grid_size;
    let start_node = Node::new(start_point, 0);
    let mut heap = BinaryHeap::from([start_node]);
    let mut seen: HashSet<Node> = HashSet::from([start_node]);
    // let mut vis = Visited::new();
    let mut best_score = 0;
    let mut counter = 0;

    while let Some(node) = heap.pop() {
        // println!(
        //     "Node: {}, {}, {}",
        //     print_tuple_2(&node.point),
        //     print_tuple_2(&node.dir),
        //     &node.cost
        // );
        // if seen.contains(&node) {
        //     continue;
        // }

        if node.point == end_point {
            println!("end found with cost {} at counter {}", node.cost, counter);
            best_score = node.cost;
            break;
        }
        if counter > 100000000 {
            println!("breaking free");
            break;
        }
        // get the neighbor nodes
        let neighbors = node.get_neighbors(&grid);
        // if best cost is set, break if encountering a node with a greater score

        // insert the neighbors to visited
        for neighbor in neighbors {
            if !seen.contains(&neighbor) {
                heap.push(neighbor);
                seen.insert(neighbor);
            }

            // seen.insert(neighbor);
            // if !seen.contains(&neighbor) {
            // }
        }
        counter += 1;
    }
    // println!("score is {}", best_score.unwrap());
    // print visited
    // println!("{}", &vis.print());
    // now that vis is built, we get all the shortest points
    // let shortest_path = vis
    //     .get_shortest_paths(start_point, end_point)
    //     .into_iter()
    //     .next()
    //     .unwrap();
    // println!("num paths: {}", shortest_paths.len());
    // for p in shortest_paths.iter() {
    //     println!("{}", print_vec_tuple_2(p));
    // }
    // let shortest_path_points = shortest_path
    //     .iter()
    //     // .flatten()
    //     .map(|n| n.point)
    //     .collect::<HashSet<_>>();
    // println!("{}", print_path_grid(&slices, &shortest_path_points));

    // (
    // shortest_path.len() - 1,
    best_score
    // the -1 is to account for the starting point being in that list
    // print_path_grid(&slices, &shortest_path_points),
    // )
}

fn parse_walls(input: &str) -> IResult<&str, Vec<(Point)>> {
    let (input, walls) = separated_list1(newline, separated_pair(i64, tag(","), i64))(input)?;
    let walls = walls
        .iter()
        .map(|(i, j)| (*j as isize, *i as isize))
        .collect();
    Ok((input, walls))
}

fn print_vec_tuple_2(i: &Vec<Point>) -> String {
    let mut s = String::new();
    s.push('[');
    for e in i {
        s.push_str(&print_tuple_2(e));
    }
    s.push(']');
    s
}

fn print_path_grid(grid: &[&[char]], path_points: &HashSet<Point>) -> String {
    grid.iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(j, c)| {
                    if path_points.contains(&(i as isize, j as isize)) {
                        if *c == '#' {
                            panic!("oh no we're on a wall in our path what the heck")
                        }
                        'O'
                    } else {
                        *c
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
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

fn in_bounds<T>(grid: &[Vec<T>], p: &Point) -> bool {
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
        let input = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let (_, walls) = parse_walls(input).unwrap();
        let res = solve(&walls[0..12], (6, 6));
        assert_eq!(22, res);
        //         assert_eq!(
        //             r"OO.#OOO
        // .O#OO#O
        // .OOO#OO
        // ...#OO#
        // ..#OO#.
        // .#.O#..
        // #.#OOOO",
        //             &res_grid
        //         );
    }
}
