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
            '.' => if _char_holder.is_empty() { 
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
                },
            _ => {
                if _char_holder.is_empty() {
                    symbols_cols.push(j);
                    continue
                } else {
                    symbols_cols.push(j);
                    line_nums.push( Num { value: _char_holder.parse::<u32>().unwrap() as usize, row: row.clone(), cols: _ind_holder.clone() });
                    _char_holder = String::from("");
                    _ind_holder.drain(0..);
                }
            }
        }
        if iter.peek().is_none() && !_char_holder.is_empty() {
            line_nums.push( Num { value: _char_holder.parse::<u32>().unwrap() as usize, row: row.clone(), cols: _ind_holder.clone() });
        }
    }
    (line_nums, symbols_cols)
}

pub fn get_valid_num(n: Num, sym_cols: &Vec<Vec<usize>>, max_len:usize, max_rows:usize) -> Option<usize> {

    let mut valid_cols = n.cols;
    valid_cols.push(usize::max(0, (valid_cols[0] as usize)-1));
    valid_cols.push(usize::min(max_len - 1, &valid_cols[valid_cols.len()-2]+1));
    let lower_ind = usize::max(n.row-1, 0);
    let upper_ind = usize::min(n.row+1, max_rows);
    let row_neighbours: &[Vec<usize>] = &sym_cols[lower_ind as usize..=upper_ind as usize];

    if row_neighbours.iter().flatten().any(|e| valid_cols.contains(e)){
        return Some(n.value)
    } else {
        return None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = 
"467..114..
*.........
....35.633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    
    let max_row_ind = 9;
    let max_len = *(&input.lines().next().unwrap().len());
    println!("{}", max_len);
    let mut nums_vec: Vec<Num> = Vec::new();
    let mut symbols_vec: Vec<Vec<usize>> = Vec::new();

    let _temp: Vec<_> = input.lines().enumerate().map(|(i,l)| {
        let (mut nums, syms) = get_groups(l, i);
        nums_vec.append(&mut nums);
        symbols_vec.push(syms);
    }).collect();
    let result: usize = nums_vec
                            .into_iter()
                            .filter_map(|n| get_valid_num(n, &symbols_vec, max_len, max_row_ind)).sum();
    assert_eq!(4361, result);
    
    }
    #[test]
    fn test_get_valid_num() {
        let n = Num { value: 10, row:4, cols: vec![0,1]};
        let sym_cols: Vec<Vec<usize>> = vec![vec![0], Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let out = get_valid_num(n, &sym_cols, 10, 4);
        assert_eq!(None, out);
    }

    #[test]
    fn test_get_groups() {
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

        let mut nums_vec: Vec<Num> = Vec::new();
        let mut symbols_vec: Vec<Vec<usize>> = Vec::new();

        let _temp: Vec<_> = input.lines().enumerate().map(|(i,l)| {
            let (mut nums, syms) = get_groups(l, i);
            nums_vec.append(&mut nums);
            symbols_vec.push(syms);
        }).collect();
        // println!("{:?}, {:?}", nums_vec, symbols_vec);
    }
}