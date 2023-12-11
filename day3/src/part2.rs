use std::usize;
use crate::Num;

pub fn get_groups(line: &str, row: usize) -> (Vec<Num>, Vec<usize>) {
     
    let mut line_nums:Vec<Num> = Vec::new();
    let mut symbols_cols:Vec<usize> = Vec::new();

    let mut _char_holder = String::from("");
    let mut _ind_holder: Vec<usize> = Vec::new();
    
    let mut iter = line.chars().enumerate().peekable();
    while let Some((j, c)) = iter.next(){
        match c {
            '0' ..= '9' => {
                _char_holder.push(c);
                _ind_holder.push(j);
            }
            '*' => if _char_holder.is_empty() { 
                symbols_cols.push(j);
                continue 
                } else { 
                    symbols_cols.push(j);
                    line_nums.push( 
                        Num { 
                            value: _char_holder.parse::<u32>().unwrap() as usize, 
                            row: row.clone(), 
                            cols: _ind_holder.clone() 
                        });
                    _char_holder = String::from("");
                    _ind_holder.drain(..);
                },
            _ => {
                if _char_holder.is_empty() {
                    continue
                } else {
                    line_nums.push( 
                        Num { 
                            value: _char_holder.parse::<u32>().unwrap() as usize, 
                            row: row.clone(), 
                            cols: _ind_holder.clone() 
                        });
                    _char_holder = String::from("");
                    _ind_holder.drain(..);
                }
            }
        }
        if iter.peek().is_none() && !_char_holder.is_empty() {
            line_nums.push( Num { value: _char_holder.parse::<u32>().unwrap() as usize, row: row.clone(), cols: _ind_holder.clone() });
        }
    }
    (line_nums, symbols_cols)
}

pub fn get_powers(n_vec: &Vec<Num>, sym_cols: &Vec<Vec<usize>>) -> usize {

    let pos_mod: [(i32, i32); 8] = [
        (-1,-1),
        (-1, 0),
        (-1, 1),
        (0,-1),
        (0,1),
        (1,-1),
        (1,0),
        (1,1),
    ];

    sym_cols.into_iter().enumerate().flat_map(|(r, row)| {
        row.iter().map(move |col | {
            let row = r.clone();
            let (val_rows, val_cols): (Vec<usize>, Vec<usize>) = pos_mod
                                                                                        .iter()
                                                                                        .map(|p| 
                                                                                            {
                                                                                                ((p.0 + row as i32) as usize, (p.1 + *col as i32) as usize)
                                                                                            }).unzip();
            
            let val_nums: Vec<usize>= n_vec
                            .iter()
                            .filter(|n| val_rows.contains(&n.row) && n.cols.iter().any(|e| val_cols.contains(e)))
                            .map(|n|n.value)
                            .collect();
            
            if val_nums.len() < 2 {
                    0
                } else {
                    val_nums.iter().product()
                }
            })
        }).sum()
    }



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() {
        let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    
    // let max_row_ind = 9;
    // let max_len = *(&input.lines().next().unwrap().len());
    // println!("{}", max_len);
    let mut nums_vec: Vec<Num> = Vec::new();
    let mut symbols_vec: Vec<Vec<usize>> = Vec::new();

    let _temp: Vec<_> = input.lines().enumerate().map(|(i,l)| {
        let (mut nums, syms) = get_groups(l, i);
        nums_vec.append(&mut nums);
        symbols_vec.push(syms);
    }).collect();
    let result: usize = get_powers(&nums_vec, &symbols_vec);
    assert_eq!(467835, result);
    
    }
//     #[test]
//     fn test_get_valid_num() {
//         let n = Num { value: 10, row:4, cols: vec![0,1]};
//         let sym_cols: Vec<Vec<usize>> = vec![vec![0], Vec::new(), Vec::new(), Vec::new(), Vec::new()];
//         let out = get_valid_num(n, &sym_cols, 10, 4);
//         assert_eq!(None, out);
//     }

//     #[test]
//     fn test_get_groups() {
//         let input = 
// "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";

//         let mut nums_vec: Vec<Num> = Vec::new();
//         let mut symbols_vec: Vec<Vec<usize>> = Vec::new();

//         let _temp: Vec<_> = input.lines().enumerate().map(|(i,l)| {
//             let (mut nums, syms) = get_groups(l, i);
//             nums_vec.append(&mut nums);
//             symbols_vec.push(syms);
//         }).collect();
//         // println!("{:?}, {:?}", nums_vec, symbols_vec);
//     }
}