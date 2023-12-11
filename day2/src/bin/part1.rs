use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("day2.txt")
        .expect("Should have been able to read the file");
    let result = solve_game(contents);
    println!("{}", result)
}

fn solve_game(text: String) -> i32 {
    let specs = (12,13,14);
    text.lines()
        .map(|l| {
            let (g_id, games) = l.split_once(":").unwrap();
            let g_id = g_id.split(" ").last().unwrap().parse::<i32>().unwrap();
            let games = games.split(";").collect::<Vec<&str>>();
            let checks: Vec<bool> = games.iter().map(|g| {
                match get_game_result(*g, specs) {
                    Some(b) => b,
                    None => false
                }   
            }).collect();
            if checks.iter().all(|x| *x) == true {
                g_id
            } else {
                0
            }
        }).sum()
}

fn get_game_result(game: &str, specs: (i8, i8, i8)) -> Option<bool> {
    let re = Regex::new(r"((?<n>\d+) (?<colour>\w+))+").unwrap();

    let blocks: Vec<bool> = re.captures_iter(game).map(|cap| {
        let num = cap.name("n").unwrap().as_str().parse::<i8>().expect("");
        let colour = cap.name("colour").unwrap().as_str();
        match colour {
            "red" => num <= specs.0,
            "green" =>num <= specs.1,
            "blue" => num <= specs.2,
            &_ => panic!("This should not occur")
        }
    }).collect();

    if blocks.iter().all(|x| *x) == true {
        Some(true)
    } else {
        None
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_game() {
        let test_input = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let output = solve_game(test_input);
        assert_eq!(8, output)
    }
    #[test]
    fn test_get_game_result() {
        let test_input = "3 blue, 4 red, 2 green";

        let output = get_game_result(test_input, (12,13,14)).unwrap();
        assert!(output)
    }
}

