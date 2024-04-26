use std::fs;
use day10::{parse_input, find_paths, follow_path};
fn main(){
    let input = fs::read_to_string("day10.txt").expect("File reading issues");
    let result = part2(input);
    println!("{:?}", result)
}

fn part2(input: String) -> i32 {
    let (start, grid) = parse_input(input);
    let path_starts = find_paths(start, &grid);
    let paths = path_starts.iter().filter_map(|step| follow_path(start, *step, &grid)).collect::<Vec<Vec<(i32,i32)>>>();
    paths.into_iter().map(|path| get_enclosed_area(path)).filter(|x| x.is_positive()).collect::<Vec<i32>>()[0]
}

fn get_enclosed_area(mut path: Vec<(i32, i32)>) -> i32 {
    let n_points = path.len() as i32;
    let mut integral = 0 as i32;
    let start = path[0].clone();
    path.push(start);
    let mut prev_pos = path[0].clone();
    for new_pos in path.iter().skip(1){
        integral += (new_pos.1 + prev_pos.1) *  (new_pos.0 - prev_pos.0);
        prev_pos = new_pos.clone();
    };
    let area = integral / 2;
    (area + 1) - (n_points/2)

}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part2() {
        let input: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let result = part2(String::from(input));
        assert_eq!(4, result)
    }
}