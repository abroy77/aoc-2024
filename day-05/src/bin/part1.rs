use std::{collections::HashMap, env, fs::read_to_string, path::PathBuf, str::FromStr};

use nom::{
    bytes::complete::{take_till, take_until},
    character::complete::{char, newline, u64},
    multi::separated_list1,
    sequence::separated_pair,
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
    let (_, (rules_map, pages)) = parse_input(&data).unwrap();
    let slices: Vec<&[usize]> = pages.iter().map(|v| v.as_slice()).collect();
    let sol = solve(&rules_map, &slices);
    println!("Solution is {}", sol);

    Ok(())
}

fn solve(rules: &HashMap<(usize, usize), bool>, pages: &[&[usize]]) -> usize {
    // we need the sum of the middle elements of the successful ones

    pages
        .iter()
        .filter(|p| check_pages_legit(p, rules))
        .map(|p| p[p.len() / 2])
        .sum()
}

fn parse_rule(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (p1, p2)) = separated_pair(u64, char('|'), u64)(input)?;
    Ok((input, (p1 as usize, p2 as usize)))
}

fn parse_pages(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, nums) = separated_list1(char(','), u64)(input)?;
    let nums = nums.into_iter().map(|x| x as usize).collect();
    Ok((input, nums))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<(usize, usize), bool>, Vec<Vec<usize>>)> {
    let (page_str, rules_str) = take_until("\n\n")(input)?;
    let (_, rules) = separated_list1(newline, parse_rule)(rules_str)?;
    let rule_map = construct_rules_dict(&rules);

    // remove the double newline from page_str
    let (page_str, _) = take_till(|c: char| c.is_numeric())(page_str)?;

    let (_, pages) = separated_list1(newline, parse_pages)(page_str)?;
    Ok(("", (rule_map, pages)))
}

fn construct_rules_dict(rules: &[(usize, usize)]) -> HashMap<(usize, usize), bool> {
    // let mut rules_dict
    let mut rules_map = HashMap::new();
    // let rev_iter = rules.iter().map(|(x,y)| (y,x));
    for (x, y) in rules.iter() {
        rules_map.insert((*x, *y), true);
        rules_map.insert((*y, *x), false);
    }
    rules_map
}

fn check_pages_legit(pages: &[usize], rules: &HashMap<(usize, usize), bool>) -> bool {
    for i in 0..pages.len() {
        for j in i..pages.len() {
            // check every pair to see if it has a rule or not. if the rule returns false then return false
            if let Some(b) = rules.get(&(pages[i], pages[j])) {
                if !b {
                    return false;
                }
            };
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (_, (rules_map, pages)) = parse_input(input).unwrap();
        let slices: Vec<&[usize]> = pages.iter().map(|v| v.as_slice()).collect();
        assert_eq!(143, solve(&rules_map, &slices));
    }
}
