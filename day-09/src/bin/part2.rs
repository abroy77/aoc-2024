use std::{collections::HashMap, env, fs::read_to_string, path::PathBuf, str::FromStr};

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
    let sol = solve(data.0, data.1);
    println!("Solution is {}", sol);

    Ok(())
}
#[derive(Debug)]
struct Chunk {
    pos: usize,
    size: usize,
}

impl Chunk {
    fn new(pos: usize, size: usize) -> Self {
        Chunk { pos, size }
    }
}

type Files = HashMap<usize, Chunk>;
type Blanks = Vec<Chunk>;
fn parse_input(input: &str) -> (Files, Blanks) {
    let nums: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut files: Files = HashMap::new();
    let mut blanks: Blanks = vec![];
    let mut pos = 0;
    for (id, w) in nums.chunks(2).enumerate() {
        let file = Chunk::new(pos, w[0]);
        files.insert(id, file);
        pos += w[0];
        let blank_len = w.get(1).unwrap_or(&0);
        let blank = Chunk::new(pos, *blank_len);
        blanks.push(blank);
        pos += blank_len;
    }

    (files, blanks)
}

fn solve(mut files: Files, mut blanks: Blanks) -> usize {
    // we start from the highest id and keep going down

    let max_id = *files.keys().max().unwrap();
    for id in (0..=max_id).rev() {
        let file = files.get_mut(&id).unwrap();
        // find a blank which would fit
        if let Some(blank_index) = blanks.iter().position(|b| b.size >= file.size) {
            // yay we found a blank to fill!
            let blank = blanks.get_mut(blank_index).unwrap();
            // we need to check if the blank we're looking for is to the right of the file
            // if it is. we've compressed all we can
            if blank.pos >= file.pos {
                break;
            }
            if blank.size == file.size {
                // move the file to the location of the blank
                file.pos = blank.pos;
                // we delete the blank
                blanks.remove(blank_index);
            } else {
                // there's some space left
                file.pos = blank.pos;
                blank.pos = file.pos + file.size;
                blank.size -= file.size;
            }
        }
    }
    let mut sum = 0;
    for (id, file) in files {
        for delta in 0..file.size {
            sum += id * (file.pos + delta)
        }
    }
    sum
} // fn parse_input(input: &str) -> Vec<Option<usize>> {
  //     let nums: Vec<usize> = input
  //         .chars()
  //         .map(|c| c.to_digit(10).unwrap() as usize)
  //         .collect();

//     let vec_size = nums.iter().sum();
//     let mut data: Vec<Option<usize>> = vec![None; vec_size];
//     let mut data_index = 0;
//     for (id, c) in nums.chunks(2).enumerate() {
//         let file_len = c[0];
//         let blank_len = c.get(1).unwrap_or(&0);
//         data.iter_mut()
//             .skip(data_index)
//             .take(file_len)
//             .for_each(|x| *x = Some(id));
//         data_index = data_index + file_len + blank_len
//     }
//     data
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "2333133121414131402";
        let data = parse_input(input);
        let (files, blanks) = data;
        assert_eq!(2858, solve(files, blanks));
    }
}
