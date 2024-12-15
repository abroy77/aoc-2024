use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::{tag, take_till},
    character::{
        complete::{newline, u64},
        is_digit, is_newline,
    },
    multi::separated_list1,
    IResult,
};

fn main() -> std::io::Result<()> {
    // get the data filepath
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Filepath not provided");
    }
    let data_path = PathBuf::from_str(&args[1]).expect("Failed to convert input to filepath");

    assert!(data_path.exists(), "data path does not exist");
    let data = read_to_string(data_path).expect("could not read datapath");
    let (_, machines) = parse_input(&data).unwrap();
    let sol = solve(machines);
    println!("Solution is {}", sol);

    Ok(())
}
type Machine = ((isize, isize), (isize, isize), (isize, isize));

fn solve(machines: Vec<Machine>) -> isize {
    machines
        .iter()
        .filter_map(solve_machine)
        .map(|presses| calculate_cost(presses.0, presses.1))
        .sum()
}

fn solve_machine(machine: &Machine) -> Option<(isize, isize)> {
    // let's write some equations
    let ((ax, ay), (bx, by), (px, py)) = machine;

    // check if the denominators are possible
    if (px * by - py * bx) % (ax * by - ay * bx) != 0 {
        // not divisible
        // no int solution
        None
    } else {
        let a = (px * by - py * bx) / (ax * by - ay * bx);
        if (px - a * ax) % bx != 0 {
            None
        } else {
            let b = (px - a * ax) / bx;
            Some((a, b))
        }
    }
}

fn calculate_cost(a: isize, b: isize) -> isize {
    a * 3 + b
}

fn parse_line(input: &str) -> IResult<&str, (isize, isize)> {
    // let (input, _) =
    let (input, _) = take_till(|c: char| is_digit(c as u8))(input)?;
    let (input, num_1) = u64(input)?;
    let (input, _) = take_till(|c: char| is_digit(c as u8))(input)?;
    let (input, num_2) = u64(input)?;
    let (input, _) = take_till(|c: char| is_newline(c as u8))(input)?;

    Ok((input, (num_1 as isize, num_2 as isize)))
}

fn parse_block(input: &str) -> IResult<&str, Machine> {
    let (input, a) = parse_line(input)?;
    let (input, _) = newline(input)?;
    let (input, b) = parse_line(input)?;
    let (input, _) = newline(input)?;
    let (input, p) = parse_line(input)?;
    Ok((input, (a, b, p)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(tag("\n\n"), parse_block)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";

        let (_, machines) = parse_input(input).unwrap();
        let ((ax, ay), (bx, by), (px, py)) = machines[0];

        assert_eq!(ax, 94);
        assert_eq!(ay, 34);
        assert_eq!(bx, 22);
        assert_eq!(by, 67);
        assert_eq!(px, 8400);
        assert_eq!(py, 5400);

        let ((ax, ay), (bx, by), (px, py)) = machines[1];

        assert_eq!(ax, 26);
        assert_eq!(ay, 66);
        assert_eq!(bx, 67);
        assert_eq!(by, 21);
        assert_eq!(px, 12748);
        assert_eq!(py, 12176);
    }

    #[test]
    fn test_parse_block() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";

        let (_, ((ax, ay), (bx, by), (px, py))) = parse_block(input).unwrap();
        assert_eq!(ax, 94);
        assert_eq!(ay, 34);
        assert_eq!(bx, 22);
        assert_eq!(by, 67);
        assert_eq!(px, 8400);
        assert_eq!(py, 5400);
    }

    #[test]
    fn test_sample() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let (_, machines) = parse_input(input).unwrap();
        assert_eq!(480, solve(machines));
    }
}
