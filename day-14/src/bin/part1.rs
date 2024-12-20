use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
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
    let (_, robots) = parse_input(&data).unwrap();
    let result = solve(&robots, 101, 103, 100);
    println!("Solution is {}", result);

    Ok(())
}

fn solve(robots: &[(Point, Point)], xlim: isize, ylim: isize, time: isize) -> isize {
    // for each robot, simulate it
    let final_points: Vec<_> = robots
        .iter()
        .map(|(p, v)| simulate_robot(*p, *v, time))
        .map(|p| (wrap_value(p.0, xlim), wrap_value(p.1, ylim)))
        .collect();

    // now we need to count the num of robots in each quadrant
    let mid_x = xlim / 2;
    let mid_y = ylim / 2;
    let mut quad1 = 0; // top left
    let mut quad2 = 0; // top right
    let mut quad3 = 0; // bottom left
    let mut quad4 = 0; // bottom right

    for p in final_points {
        let (x, y) = p;
        if x < mid_x {
            if y < mid_y {
                quad1 += 1;
            } else if y > mid_y {
                quad3 += 1;
            }
        } else if x > mid_x {
            if y < mid_y {
                quad2 += 1;
            } else if y > mid_y {
                quad4 += 1;
            }
        }
    }

    quad1 * quad2 * quad3 * quad4
}

fn simulate_robot(p: Point, v: Point, t: isize) -> Point {
    add_points(p, scale_point(v, t))
}

fn add_points(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

fn scale_point(p: Point, k: isize) -> Point {
    (p.0 * k, p.1 * k)
}

fn wrap_value(x: isize, lim: isize) -> isize {
    let mut res = x % lim;
    if res < 0 {
        res += lim;
    }
    res
}

fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, ((px, py), (vx, vy))) = separated_pair(
        preceded(tag("p="), separated_pair(i64, tag(","), i64)),
        space1,
        preceded(tag("v="), separated_pair(i64, tag(","), i64)),
    )(input)?;

    Ok((
        input,
        ((px as isize, py as isize), (vx as isize, vy as isize)),
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Point, Point)>> {
    separated_list1(newline, parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = r"p=0,4 v=3,-3";
        let ans = ((0, 4), (3, -3));
        let (_, res) = parse_line(input).unwrap();
        assert_eq!(ans, res);
    }

    #[test]
    fn test_wrapping() {
        let lim = 3;
        let inputs = [-5, -2, 2, 5, 7, -7];
        let answers = [1, 1, 2, 2, 1, 2];
        for (i, a) in inputs.iter().zip(answers.iter()) {
            assert_eq!(*a, wrap_value(*i, lim));
        }
    }

    #[test]
    fn test_sample() {
        let input = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let xlim = 11;
        let ylim = 7;
        let time = 100;
        let (_, robots) = parse_input(input).unwrap();
        assert_eq!(12, solve(&robots, xlim, ylim, time));
    }
}
