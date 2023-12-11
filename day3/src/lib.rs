mod part1;
mod part2;

#[derive(Debug)]
pub struct Num{
    value: usize,
    row: usize,
    cols: Vec<usize>
}

// impl Clone for Num {
//     fn clone(&self) -> Num {
//         *self
//     }
// }

pub fn part1(s: String) {
    
    let max_len = *(&s.lines().next().unwrap().len());
    let max_rows:usize = 139;
    let mut nums_vec: Vec<Num> = Vec::new();
    let mut symbols_vec: Vec<Vec<usize>> = Vec::new();

    let _temp: Vec<_> = s.lines().enumerate().map(|(i,l)| {
        let (mut nums, syms) = part1::get_groups(l, i);
        nums_vec.append(&mut nums);
        symbols_vec.push(syms);
    }).collect();

    let result: usize = nums_vec.into_iter().filter_map(|n| part1::get_valid_num(n, &symbols_vec, max_len, max_rows)).sum();
    println!("{}", result)
}

pub fn part2(s: String) {
    
    let mut nums_vec: Vec<Num> = Vec::new();
    let mut symbols_vec: Vec<Vec<usize>> = Vec::new();

    let _temp: Vec<_> = s.lines().enumerate().map(|(i,l)| {
        let (mut nums, syms) = part2::get_groups(l, i);
        nums_vec.append(&mut nums);
        symbols_vec.push(syms);
    }).collect();
    let result: usize = part2::get_powers(&nums_vec, &symbols_vec);
    println!("{}", result)
}