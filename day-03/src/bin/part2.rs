use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, i32, newline},
    combinator::{rest, value},
    multi::{many0, many_till, separated_list1},
    sequence::{delimited, preceded, separated_pair},
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
    let (_, parsed_data) = parse_input(&data).unwrap();
    let slices: Vec<&[(i32, i32)]> = parsed_data.iter().map(|v| v.as_slice()).collect();
    let result = solve(&slices);
    println!("Solution is {}", result);

    Ok(())
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    // delimited(char('('), separated_pair(i32, char(','), i32), char(')'))(input)
    preceded(
        tag("mul"),
        delimited(char('('), separated_pair(i32, char(','), i32), char(')')),
    )(input)
}

fn find_mul(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, (_, mul_match)) = many_till(anychar, parse_mul)(input)?;
    Ok((input, mul_match))
}
fn parse_muls(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many0(find_mul)(input)
}
fn parse_dont_chunk(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("don't()")(input)?;
    let (input, _) = alt((take_until("do()"), rest))(input)?;

    Ok((input, ""))
}

fn parse_do_chunk(input: &str) -> IResult<&str, &str> {
    alt((take_until("don't()"), rest))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let mut muls = Vec::new();
    let mut remainder = input;

    while !remainder.is_empty() {
        // assume we're starting in the 'do()' zone
        let (after_do, to_analyze) = parse_do_chunk(remainder)?;
        let (_, part_muls) = parse_muls(to_analyze)?;
        muls.extend(part_muls);
        if after_do.is_empty() {
            break;
        } else {
            // we hit the don't section/ uh oh
            let (after_dont, _) = parse_dont_chunk(after_do)?;
            remainder = after_dont;
        }
    }
    Ok((remainder, muls))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(i32, i32)>>> {
    separated_list1(newline, parse_line)(input)
}

fn solve(data: &[&[(i32, i32)]]) -> i32 {
    data.iter()
        .flat_map(|line| line.iter().map(|(x, y)| x * y))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mul() {
        let input = r"mul(2,4)";
        let numbers = (2, 4);
        let (_, parsed_result) = parse_mul(input).unwrap();
        assert_eq!(numbers, parsed_result);
    }

    #[test]
    fn test_find_mul() {
        let input = r"estestmul(2,4)";
        let numbers = (2, 4);
        let (_, parsed_result) = find_mul(input).unwrap();
        assert_eq!(numbers, parsed_result);
    }

    #[test]
    fn test_parse_line() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let numbers = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let (_, parsed_result) = parse_line(input).unwrap();
        assert_eq!(numbers, parsed_result);
    }
    #[test]
    fn test_parse_till_dont() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let numbers = vec![(2, 4), (8, 5)];
        let (_, nums) = parse_line(input).unwrap();
        assert_eq!(numbers, nums);
    }
    #[test]
    fn test_sample() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let sol = 48;
        let (_, parsed_result) = parse_input(input).unwrap();
        let slices: Vec<&[(i32, i32)]> = parsed_result.iter().map(|v| v.as_slice()).collect();
        let out = solve(&slices);
        assert_eq!(sol, out);
    }
}
