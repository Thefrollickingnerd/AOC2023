use std::fs;
use day10::{parse_input, find_paths, follow_path};
fn main(){
    let input = fs::read_to_string("day10.txt").expect("File reading issues");
    let result = part1(input);
    println!("{result}")
}

fn part1(input: String) -> usize {
    let (start, grid) = parse_input(input);
    let path_starts = find_paths(start, &grid);
    let paths = path_starts.iter().filter_map(|step| follow_path(start, *step, &grid)).collect::<Vec<Vec<(i32,i32)>>>();
    paths[0].len() / 2
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let (start, grid) = parse_input(String::from(input));
        let paths = find_paths(start, &grid);
        println!("{:?}",&start);
        println!("{:?}",&paths);
        // dbg!(&grid);
        let path1 = follow_path(start, paths[0], &grid).unwrap();
        println!("{:?}",&path1);
        println!("{:?}",&path1.len());
        println!("{:?}",&path1[&path1.len()/2]);
    }
}

