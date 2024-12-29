use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
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
    let (_, (patterns, designs)) = parse_input(&data).unwrap();
    let result = solve(&patterns, designs);
    println!("Solution is {}", result);

    Ok(())
}

fn solve(patterns: &HashSet<&str>, designs: Vec<&str>) -> usize {
    let mut seen_designs = HashMap::new();
    let max_pattern_length = patterns.iter().map(|x| x.len()).max().unwrap();
    designs
        .into_iter()
        .filter(|design| {
            is_design_possible(design, patterns, &mut seen_designs, max_pattern_length)
        })
        .count()
}

fn is_design_possible<'a>(
    design: &'a str,
    patterns: &HashSet<&str>,
    seen_designs: &mut HashMap<&'a str, bool>,
    max_pattern_len: usize,
) -> bool {
    if design.is_empty() {
        return true;
    }
    match seen_designs.get(design) {
        Some(b) => *b,
        None => {
            // no match is seen before. need to do the matching
            // match diff lengths in the design
            for i in 1..=design.len().min(max_pattern_len) {
                let (sub_design, remaining) = design.split_at(i);
                if patterns.contains(sub_design)
                    && is_design_possible(remaining, patterns, seen_designs, max_pattern_len)
                {
                    seen_designs.insert(design, true);
                    return true;
                }
            }
            seen_designs.insert(design, false);
            false
        }
    }
}

fn parse_patterns(input: &str) -> IResult<&str, HashSet<&str>> {
    let (input, patterns) = separated_list1(tag(", "), alphanumeric1)(input)?;
    let patterns = patterns.into_iter().collect();
    Ok((input, patterns))
}

fn parse_designs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(newline, alphanumeric1)(input)
}

fn parse_input(input: &str) -> IResult<&str, (HashSet<&str>, Vec<&str>)> {
    separated_pair(parse_patterns, tag("\n\n"), parse_designs)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
    }
    #[test]
    fn test_parsing() {
        let input = input();
        let (_, (patterns, designs)) = parse_input(input).unwrap();
        let known_patterns = HashSet::from(["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        let known_designs = vec![
            "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
        ];

        assert_eq!(patterns, known_patterns);
        assert_eq!(designs, known_designs);
    }

    #[test]
    fn test_sample() {
        let input = input();
        let (_, (patterns, designs)) = parse_input(input).unwrap();
        assert_eq!(6, solve(&patterns, designs));
    }
}
