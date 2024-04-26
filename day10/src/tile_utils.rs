#[derive(Debug)]
pub enum Tile {
    Start,
    Soil,
    Pipe { 
        ptype:char, 
        pos1: (i32, i32),
        pos2: (i32, i32)
    }
}

impl Tile {
    pub fn new(c: char, r: i32, col: i32) -> Tile {
        match c {
            '.' => Tile::Soil,
            'S' => Tile::Start,
            _ => {
                let (p1, p2) = match c {
                    '-' => ((r, col.saturating_sub(1)), (r, col.saturating_add(1))),
                    '|' => ((r.saturating_sub(1), col), (r.saturating_add(1), col)),
                    'F' => ((r.saturating_add(1), col), (r, col .saturating_add(1))),
                    '7' => ((r.saturating_add(1), col), (r, col .saturating_sub(1))),
                    'J' => ((r.saturating_sub(1), col), (r, col .saturating_sub(1))),
                    'L' => ((r.saturating_sub(1), col), (r, col .saturating_add(1))),
                    _ => panic!("Invalid pipe passed"),
                };
                Tile::Pipe { ptype: c, pos1: p1, pos2: p2 }
            }
        }       
    }
}
impl Default for Tile {
    fn default() -> Self { Tile::Soil }
}

pub fn check_valid_connection(
    cur_pos:(i32, i32), p1:(i32, i32), p2:(i32, i32)) -> Option<(i32, i32)> {
    if cur_pos == p1 {
        Some(p2)
    } else if cur_pos == p2 {
        Some(p1)
    } else {
        None
    }
} 