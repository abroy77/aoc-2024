use core::panic;
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
        pos = move_thing_general(&mut slices, mv, pos);
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
    let (input, grid) = separated_list1(line_ending, many1(none_of("\n\r")))(input)?;
    let grid = grid
        .into_iter()
        .map(|v| {
            v.into_iter()
                .flat_map(|c| match c {
                    '.' => ['.', '.'].into_iter(),
                    '@' => ['@', '.'].into_iter(),
                    '#' => ['#', '#'].into_iter(),
                    'O' => ['[', ']'].into_iter(),
                    _ => panic!("encountered unknown char in parsing"),
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    Ok((input, grid))
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

// fn move_thing_vertically(

fn move_wall_vertically(grid: &mut [&mut [char]], dir: Point, thing_pos: Point) -> Point {
    assert!([(1, 0), (-1, 0)].contains(&dir));
    let current_element_copy = grid[thing_pos.0 as usize][thing_pos.1 as usize];
    dbg!(current_element_copy);
    assert!(current_element_copy == '[' || current_element_copy == ']');
    let paired_pos = if current_element_copy == '[' {
        (thing_pos.0, thing_pos.1 + 1)
    } else {
        (thing_pos.0, thing_pos.1 - 1)
    };

    let paired_element_copy = grid[paired_pos.0 as usize][paired_pos.1 as usize];

    // recursive function
    // returns the position the thing was moved to
    let next_pos = add_points(thing_pos, dir);
    let next_p_pos = add_points(paired_pos, dir);
    match (
        grid[next_pos.0 as usize][next_pos.1 as usize],
        grid[next_p_pos.0 as usize][next_p_pos.1 as usize],
    ) {
        ('.', '.') => {
            // free space to move into
            grid[next_pos.0 as usize][next_pos.1 as usize] = current_element_copy;
            grid[next_p_pos.0 as usize][next_p_pos.1 as usize] = paired_element_copy;
            grid[thing_pos.0 as usize][thing_pos.1 as usize] = '.';
            grid[paired_pos.0 as usize][paired_pos.1 as usize] = '.';
            next_pos
        }
        ('#', _) | (_, '#') => {
            //it's a wall. no movement
            thing_pos
        }
        ('.', '[') => {
            // now try to push the right wall
            // first deduce where the right wall is relative to us
            let next_pos_to_push = if current_element_copy == '[' {
                next_p_pos
            } else {
                next_pos
            };
            let moved_pos = move_wall_vertically(grid, dir, next_pos_to_push);
            if moved_pos == next_pos_to_push {
                // oh no, we could not move. ripperoni
                thing_pos
            } else {
                // yay we moved
                grid[next_pos.0 as usize][next_pos.1 as usize] = current_element_copy;
                grid[next_p_pos.0 as usize][next_p_pos.1 as usize] = paired_element_copy;
                grid[thing_pos.0 as usize][thing_pos.1 as usize] = '.';
                grid[paired_pos.0 as usize][paired_pos.1 as usize] = '.';
                next_pos
            }
        }
        (']', '.') => {
            // now try to push the left wall
            // first deduce where the right wall is relative to us
            let next_pos_to_push = if current_element_copy == '[' {
                next_pos
            } else {
                next_p_pos
            };
            let moved_pos = move_wall_vertically(grid, dir, next_pos_to_push);
            if moved_pos == next_pos_to_push {
                // oh no, we could not move. ripperoni
                thing_pos
            } else {
                // yay we moved
                grid[next_pos.0 as usize][next_pos.1 as usize] = current_element_copy;
                grid[next_p_pos.0 as usize][next_p_pos.1 as usize] = paired_element_copy;
                grid[thing_pos.0 as usize][thing_pos.1 as usize] = '.';
                grid[paired_pos.0 as usize][paired_pos.1 as usize] = '.';
                next_pos
            }
        }

        ('[', ']') => {
            // now try to push the wall right ahead of us
            let moved_pos = move_wall_vertically(grid, dir, next_pos);
            if moved_pos == next_pos {
                // oh no, we could not move. ripperoni
                thing_pos
            } else {
                // yay we moved
                grid[next_pos.0 as usize][next_pos.1 as usize] = current_element_copy;
                grid[next_p_pos.0 as usize][next_p_pos.1 as usize] = paired_element_copy;
                grid[thing_pos.0 as usize][thing_pos.1 as usize] = '.';
                grid[paired_pos.0 as usize][paired_pos.1 as usize] = '.';
                next_pos
            }
        }
        (']', '[') => {
            // this is the roughest case. only move ahead if both boxes move ahead
            let moved_pos_1 = move_wall_vertically(grid, dir, next_pos);
            let moved_pos_2 = move_wall_vertically(grid, dir, next_p_pos);
            if moved_pos_1 == next_pos || moved_pos_2 == next_p_pos {
                // oh no, we could not move. ripperoni
                thing_pos
            } else {
                // yay we moved
                grid[next_pos.0 as usize][next_pos.1 as usize] = current_element_copy;
                grid[next_p_pos.0 as usize][next_p_pos.1 as usize] = paired_element_copy;
                grid[thing_pos.0 as usize][thing_pos.1 as usize] = '.';
                grid[paired_pos.0 as usize][paired_pos.1 as usize] = '.';
                next_pos
            }
        }
        _ => panic!("unknown element combo encountered"),
    }
}

fn move_thing_general(grid: &mut [&mut [char]], dir: Point, thing_pos: Point) -> Point {
    let current_element_copy = grid[thing_pos.0 as usize][thing_pos.1 as usize];
    if current_element_copy != '@' {
        // if we're not moving the robot, we better be moving a box horizontally
        assert!([(0, 1), (0, -1)].contains(&dir));
    }
    // recursive function
    // returns the position the thing was moved to
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
        '[' | ']' => {
            // diff cases if moving vertically or horizontally
            let moved_pos = match dir {
                (1, 0) | (-1, 0) => move_wall_vertically(grid, dir, next_pos),
                (0, -1) | (0, 1) => move_thing_general(grid, dir, next_pos),
                _ => panic!("we should not get a dir that is not one of these"),
            };

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
