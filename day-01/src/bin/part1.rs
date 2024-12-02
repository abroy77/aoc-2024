use nom::{
    character::complete::{i32, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

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
    let result = solve(&parsed_data);
    println!("Solution is {}", result);

    Ok(())
}

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, space1, i32)(input)
}
fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(newline, parse_line)(input)
}

fn solve(data: &[(i32, i32)]) -> u32 {
    let mut first_values: Vec<i32> = data.iter().map(|x| x.0).collect();
    first_values.sort();
    let mut second_values: Vec<i32> = data.iter().map(|x| x.1).collect();
    second_values.sort();

    first_values
        .iter()
        .zip(second_values.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = r"3   4";
        let numbers = (3, 4);
        let (_, parsed_result) = parse_line(input).unwrap();
        assert_eq!(numbers, parsed_result);
    }
    #[test]
    fn test_parse_input() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";
        let numbers = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        let (_, parsed_result) = parse_input(input).unwrap();
        assert_eq!(numbers, parsed_result);
    }
}
