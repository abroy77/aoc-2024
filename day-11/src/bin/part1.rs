use std::{env, fs::read_to_string, path::PathBuf, str::FromStr};

use nom::{
    character::complete::{space1, u64},
    multi::separated_list1,
    IResult,
};
const ITERATIONS: usize = 25;
fn main() -> std::io::Result<()> {
    // get the data filepath
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Filepath not provided");
    }
    let data_path = PathBuf::from_str(&args[1]).expect("Failed to convert input to filepath");

    assert!(data_path.exists(), "data path does not exist");
    let data = read_to_string(data_path).expect("could not read datapath");
    let (_, data) = parse_input(&data).unwrap();
    let sol = solve(data);
    println!("Solution is {}", sol);

    Ok(())
}

fn parse_input(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, data) = separated_list1(space1, u64)(input)?;
    let data = data.iter().map(|x| *x as usize).collect();
    Ok((input, data))
}

fn get_num_digits(num: usize) -> usize {
    if num == 0 {
        1
    } else {
        let mut len = 0;
        let mut i = num;
        while i > 0 {
            i /= 10;
            len += 1;
        }
        len
    }
}

fn split_even_digits(num: usize) -> (usize, usize) {
    let num_digits = get_num_digits(num);
    assert_eq!(0, num_digits % 2);

    let second_part = num % (10_usize.pow(num_digits as u32 / 2));

    let first_part = num / (10_usize.pow(num_digits as u32 / 2));

    (first_part, second_part)
}

fn solve(mut data: Vec<usize>) -> usize {
    for _ in 0..ITERATIONS {
        let mut new_vec = Vec::with_capacity(data.len() * 2);
        for e in data.iter() {
            if *e == 0 {
                new_vec.push(1);
            } else if get_num_digits(*e) % 2 == 0 {
                let (f, s) = split_even_digits(*e);
                new_vec.push(f);
                new_vec.push(s);
            } else {
                new_vec.push(e * 2024)
            }
        }
        data = new_vec;
    }

    data.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_even_digits() {
        let data = [24, 5678, 890689, 98765438];
        let ans = [(2, 4), (56, 78), (890, 689), (9876, 5438)];
        for (i, (f, s)) in data.iter().zip(ans.iter()) {
            let (first, second) = split_even_digits(*i);
            assert_eq!(first, *f);
            assert_eq!(second, *s);
        }
    }

    #[test]
    fn test_1_blink() {
        let input = r"125 17";
        let (_, data) = parse_input(input).unwrap();
        assert_eq!(55312, solve(data));
    }
}
