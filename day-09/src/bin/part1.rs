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
    let data = parse_input(&data);
    let sol = solve(data);
    println!("Solution is {}", sol);

    Ok(())
}

fn solve(mut data: Vec<Option<usize>>) -> usize {
    // we will need a double pointer and to swap things around
    let mut file_index = 0;
    let mut blank_index = data.len() - 1;

    while file_index < blank_index {
        if data[file_index].is_some() {
            // occupado
            file_index += 1;
            continue;
        } else {
            // aha! a free space
            if data[blank_index].is_some() {
                // swap em like it's hot
                data.swap(file_index, blank_index);
            } else {
                // nothing to swap in here.
                blank_index -= 1;
                continue;
            }
        }
    }
    data.iter()
        .filter_map(|e| *e)
        .enumerate()
        .map(|(i, x)| x * i)
        .sum()
}

fn parse_input(input: &str) -> Vec<Option<usize>> {
    let nums: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let vec_size = nums.iter().sum();
    let mut data: Vec<Option<usize>> = vec![None; vec_size];
    let mut data_index = 0;
    for (id, c) in nums.chunks(2).enumerate() {
        let file_len = c[0];
        let blank_len = c.get(1).unwrap_or(&0);
        data.iter_mut()
            .skip(data_index)
            .take(file_len)
            .for_each(|x| *x = Some(id));
        data_index = data_index + file_len + blank_len
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "2333133121414131402";
        let data = parse_input(input);
        let result_chars = "00...111...2...333.44.5555.6666.777.888899";
        let result_chars: Vec<Option<usize>> = result_chars
            .chars()
            .map(|c| {
                if c.is_numeric() {
                    Some(c.to_digit(10).unwrap() as usize)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(result_chars, data);
    }

    #[test]
    fn test_sample() {
        let input = "2333133121414131402";
        let data = parse_input(input);
        assert_eq!(1928, solve(data));
    }
}
