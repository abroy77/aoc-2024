use nom::{
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    IResult,
};
use std::{
    borrow::Borrow,
    cmp::{self, Ordering},
    collections::HashMap,
    env,
    fs::read_to_string,
    path::PathBuf,
    str::FromStr,
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
    let slices: Vec<&[u64]> = parsed_data.iter().map(|v| v.as_slice()).collect();
    let result = solve(&slices);
    println!("Solution is {}", result);

    Ok(())
}

fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(newline, parse_line)(input)
}

fn solve(data: &[&[u64]]) -> u64 {
    data.iter()
        .map(|v| is_report_safe(v))
        .filter(|x| *x)
        .count() as u64
}

fn get_report_order(report: &[u64]) -> Ordering {
    // look at the first 4 elements and try to deduce the order from there. we are looking for at least 2 strict relationships
    // 4 elements have 3 comparisons.
    let mut counts = HashMap::from([
        (cmp::Ordering::Greater, 0),
        (cmp::Ordering::Less, 0),
        (cmp::Ordering::Equal, 0),
    ]);

    report
        .windows(2)
        .take(3)
        .map(|w| w[0].cmp(&w[1]))
        .for_each(|ordering| {
            if let Some(count) = counts.get_mut(&ordering) {
                *count += 1;
            }
        });

    counts.iter().max_by_key(|entry| entry.1).unwrap().0.clone()
}

fn is_report_safe(report: &[u64]) -> Option<usize> {
    match get_report_order(report) {
        cmp::Ordering::Less => 
    }
    
    
    if report.is_sorted_by(|a, b| a.ge(b)) {
        report.windows(2).enumerate().find_map(|(i, w)| {
            let diff = w[0] - w[1];
            if (1..=3).contains(&diff) {
                None
            } else {
                Some(i)
            }
        })
    } else if report.is_sorted_by(|a, b| b.ge(a)) {
        report.windows(2).enumerate().find_map(|(i, w)| {
            let diff = w[1] - w[0];
            if (1..=3).contains(&diff) {
                None
            } else {
                Some(i)
            }
        })
    } else {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = r"49 52 53 55 58 59 61 61";
        let numbers = vec![49, 52, 53, 55, 58, 59, 61, 61];
        let (_, parsed_result) = parse_line(input).unwrap();
        assert_eq!(numbers, parsed_result);
    }
    #[test]
    fn test_sample() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let (_, data) = parse_input(input).unwrap();
        let slices: Vec<&[u64]> = data.iter().map(|v| v.as_slice()).collect();
        assert_eq!(2, solve(&slices));
    }
}
