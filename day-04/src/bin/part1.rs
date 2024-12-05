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
    let grid = parse_grid(&data);
    let sol = solve(grid);
    println!("Solution is {}", sol);

    Ok(())
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        result.push(row);
    }
    for i in 0..result.len() - 1 {
        assert_eq!(result[i].len(), result[i + 1].len());
    }
    result
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn solve(input: Vec<Vec<char>>) -> usize {
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut count = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if input[i][j] != 'X' {
                continue;
            }

            for (dr, dc) in DIRECTIONS {
                let (end_i, end_j) = (i as isize + 3 * dr, j as isize + 3 * dc);
                if end_j >= 0
                    && end_j < num_cols as isize
                    && end_i >= 0
                    && end_i < num_rows as isize
                {
                    let i = i as isize;
                    let j = j as isize;

                    if input[(i + dr) as usize][(j + dc) as usize] == 'M'
                        && input[(i + 2 * dr) as usize][(j + 2 * dc) as usize] == 'A'
                        && input[end_i as usize][end_j as usize] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use std::char;

    use super::*;

    #[test]
    fn test_parse() {
        let input = r"ab
cd";
        let answer: Vec<Vec<char>> = vec!["ab".chars().collect(), "cd".chars().collect()];
        let result = parse_grid(input);
        assert_eq!(answer, result);
    }

    #[test]
    fn test_sample() {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let grid = parse_grid(input);
        assert_eq!(18, solve(grid));
    }
}
