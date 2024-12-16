use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, line_ending, none_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

type Point = (isize, isize);

fn main() -> std::io::Result<()> {
    // get the data filepath
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Filepath not provided");
    }
    let data_path = PathBuf::from_str(&args[1]).expect("Failed to convert input to filepath");

    assert!(data_path.exists(), "data path does not exist");
    let data = read_to_string(data_path).expect("could not read datapath");
    let (_, (grid, moves)) = parse_input(&data).unwrap();
    let result = solve(grid, moves);

    println!("Solution is {}", result);

    Ok(())
}

fn solve(mut grid: Vec<Vec<char>>, moves: Vec<Point>) -> usize {
    let slices: Vec<&[char]> = grid.iter().map(|v| v.as_slice()).collect();
    let mut pos = get_start_point(&slices);
    // now make a mutable slice ref
    let mut slices: Vec<&mut [char]> = grid.iter_mut().map(|v| v.as_mut_slice()).collect();

    for mv in moves.into_iter() {
        pos = move_thing(&mut slices, mv, pos);
    }

    // now we need to score the system

    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, c)| match *c {
                'O' => Some((i, j)),
                _ => None,
            })
        })
        .map(|(i, j)| 100 * i + j)
        .sum()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Point>)> {
    separated_pair(parse_grid, tag("\n\n"), parse_moves)(input)
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(none_of("\n\r")))(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, moves) = many1(anychar)(input)?;
    let moves = moves
        .into_iter()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("unknown char in moves"),
        })
        .collect();

    Ok((input, moves))
}

fn move_thing(grid: &mut [&mut [char]], dir: Point, thing_pos: Point) -> Point {
    // recursive function
    // returns the position the thing was moved to
    let current_element_copy = grid[thing_pos.0 as usize][thing_pos.1 as usize];
    let next_pos = add_points(thing_pos, dir);
    match grid[next_pos.0 as usize][next_pos.1 as usize] {
        '.' => {
            // free space to move into
            grid[next_pos.0 as usize][next_pos.1 as usize] = current_element_copy;
            grid[thing_pos.0 as usize][thing_pos.1 as usize] = '.';
            next_pos
        }
        '#' => {
            //it's a wall. no movement
            thing_pos
        }
        'O' => {
            // things get interesting here.
            // this is the recursive case
            let moved_pos = move_thing(grid, dir, next_pos);
            if moved_pos == next_pos {
                // the box did not move.
                thing_pos
            } else {
                // we did move. so now we can actually move as if it was a '.'

                grid[next_pos.0 as usize][next_pos.1 as usize] = current_element_copy;
                grid[thing_pos.0 as usize][thing_pos.1 as usize] = '.';
                next_pos
            }
        }
        _ => panic!("unknown element encountered"),
    }
}

fn get_start_point(grid: &[&[char]]) -> Point {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
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
    fn test_parse_grid() {
        let input = r"ab
cd";
        let (_, grid) = parse_grid(input).unwrap();
        assert_eq!(vec![vec!['a', 'b'], vec!['c', 'd']], grid);
    }

    #[test]
    fn test_small_sample() {
        let input = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let (_, (grid, moves)) = parse_input(input).unwrap();
        assert_eq!(2028, solve(grid, moves));
    }
    #[test]
    fn test_big_sample() {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (_, (grid, moves)) = parse_input(input).unwrap();
        assert_eq!(10092, solve(grid, moves));
    }
}
