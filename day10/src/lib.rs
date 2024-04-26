extern crate rstest;
pub mod tile_utils;

use std::collections::BTreeMap;
use self::tile_utils::{check_valid_connection, Tile};

pub fn parse_input(s: String) -> ((i32, i32), BTreeMap<(i32, i32), Tile>){
    let mut start: (i32, i32) = (0,0);
    let grid = s.split("\n").enumerate().flat_map(|(r,l)| {
        l.chars().enumerate().map(move |(col, c)| {
            let t = Tile::new(c, r as i32, col as i32);
            ((r as i32,col as i32), t)
            })
    })
    .inspect(|((r, col), t)| match t {
        Tile::Start => start = (*r as i32, *col as i32),
        _ => {}
    }).collect::<BTreeMap<(i32, i32), Tile>>();
    (start, grid)
}

pub fn take_step(
    current: (i32, i32), 
    step: (i32, i32), 
    grid: &BTreeMap<(i32, i32), Tile>) 
    -> Option<((i32, i32), (i32, i32))>{
    
    let (next_cur, next_tile) = grid.get_key_value(&step).unwrap_or((&(0,0),&Tile::Soil));
    match next_tile {
        Tile::Soil => None,
        Tile::Start => None,
        Tile::Pipe { ptype:_, pos1:p1, pos2:p2 } => match check_valid_connection(current, *p1, *p2) {
            None => None,
            Some(next_step) => Some((*next_cur, next_step))
        }
    }
}

pub fn find_paths(
    start: (i32, i32), 
    grid: &BTreeMap<(i32, i32), Tile>) -> Vec<(i32, i32)> {

    let mut paths: Vec<(i32, i32)> = Vec::new();
    let modifier: Vec<(i32, i32)> = Vec::from(
        [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]
    );

    for m in modifier.iter(){
        let neighbour = (start.0 + m.0, start.1 + m.1);
        match grid.get(&neighbour).unwrap_or(&Tile::Soil) {
            Tile::Soil => continue,
            Tile::Pipe { ptype:_, pos1:p1, pos2:p2 } => match check_valid_connection(start, *p1, *p2) {
                Some(_) => paths.push(neighbour),
                None => continue
            },
            Tile::Start => continue,
        }
    }
    return paths;
}

pub fn follow_path(start: (i32, i32), first_step: (i32, i32), grid: &BTreeMap<(i32, i32), Tile>) -> Option<Vec<(i32, i32)>>{
    let mut cur_position = start;
    let mut next_step = first_step;
    let mut path: Vec<(i32, i32)> = Vec::from([cur_position]);
    while next_step != start {
        match take_step(cur_position, next_step, grid) {
            None => return None,
            Some((p1, p2)) => {
                path.push(next_step);
                cur_position = p1;
                next_step = p2;
            }
        }
    }
    return Some(path)
}