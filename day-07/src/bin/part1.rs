use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};
#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

fn apply_op(x: usize, y: usize, op: &Operation) -> usize {
    match op {
        Operation::Add => x + y,
        Operation::Multiply => x * y,
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
    let (_, lines) = parse_input(&data).unwrap();
    let sol = solve(lines);
    println!("Solution is {}", sol);

    Ok(())
}

fn solve(lines: Vec<(usize, Vec<usize>)>) -> usize {
    lines.into_iter().map(|p| solve_line(p.0, p.1)).sum()
}

fn solve_line(res: usize, operands: Vec<usize>) -> usize {
    let perms: Vec<_> = repeat_n(
        [Operation::Add, Operation::Multiply].iter(),
        operands.len() - 1,
    )
    .multi_cartesian_product()
    .collect();

    for p in perms.iter() {
        let p_res = operands
            .iter()
            .skip(1)
            .zip(p.iter())
            .fold(operands[0], |acc, (operand, operator)| {
                apply_op(acc, *operand, operator)
            });
        if p_res == res {
            return res;
        }
    }

    0
}

fn parse_line(input: &str) -> IResult<&str, (usize, Vec<usize>)> {
    let (input, (res, operands)) =
        separated_pair(u64, tag(": "), separated_list1(space1, u64))(input)?;
    let res = res as usize;
    let operands = operands.iter().map(|x| *x as usize).collect();
    Ok((input, (res, operands)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(usize, Vec<usize>)>> {
    separated_list1(newline, parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let (_, parsed_data) = parse_input(input).unwrap();
        assert_eq!(3749, solve(parsed_data));
    }

    #[test]
    fn test_292() {
        let input = r"292: 11 6 16 20";

        let (_, parsed_data) = parse_input(input).unwrap();
        assert_eq!(292, solve(parsed_data));
    }
}
