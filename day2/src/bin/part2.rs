use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("day2.txt")
        .expect("Should have been able to read the file");
    let result = solve_game(contents);
    println!("{}", result)
}

fn solve_game(text: String) -> i32 {
    text.lines()
        .map(|l| {
            let mut min_blocks = vec![0,0,0];
            let (_, games) = l.split_once(":").unwrap();
            let games = games.split(";").collect::<Vec<&str>>();
            let _checks: Vec<_> = games.iter().map(|g| {
                for (i,n) in get_game_result(*g).iter().enumerate(){
                    if min_blocks[i] < *n {min_blocks[i] = *n}
                }     
            }).collect();
            min_blocks[0] * min_blocks[1] * min_blocks[2]
        }).sum()
}

fn get_game_result(game: &str) -> Vec<i32> {
    let re = Regex::new(r"((?<n>\d+) (?<colour>\w+))+").unwrap();
    let mut rgb: Vec<i32> = vec![0,0,0];

    let _blocks: Vec<_> = re.captures_iter(game).map(|cap| {
        let num = cap.name("n").unwrap().as_str().parse::<i32>().expect("");
        let colour = cap.name("colour").unwrap().as_str();
        match colour {
            "red" => rgb[0] = num,
            "green" => rgb[1] = num,
            "blue" => rgb[2] = num,
            &_ => panic!("This should not occur")
        }
    }).collect();
    rgb
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_game() {
        let test_input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
");
        let output = solve_game(test_input);
        assert_eq!(48 + 12, output)
    }
    #[test]
    fn test_get_game_result() {
        let test_input = "3 blue, 4 red, 2 green";

        let output = get_game_result(test_input);
        assert_eq!(output, [4,2,3])
    }
}

