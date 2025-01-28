// given an array of squares, solve for the correct orientation and list moves to achieve it
// struct for a tile
// struct for the table
// struct for a move type
#![allow(dead_code)]

use std::fmt::Display;

use rand::{thread_rng, RngCore};

#[derive(Default, PartialEq, Clone, Debug)]
enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Clone, Debug)]
struct Edge {
    dir: Dir,
    val: char,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Edge {
    fn new(dir: Dir, val: char) -> Self {
        Self { dir, val }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
struct Tile {
    id: u32,
    top: Option<Edge>,
    bottom: Option<Edge>,
    right: Option<Edge>,
    left: Option<Edge>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile {:#?}\ntop: {:#?}\nbottom: {:#?}\nright: {:#?}\nleft: {:#?}\n",
            self.id, self.top, self.bottom, self.right, self.left
        )
    }
}

impl Tile {
    fn new(
        id: u32,
        top: Option<Edge>,
        bottom: Option<Edge>,
        right: Option<Edge>,
        left: Option<Edge>,
    ) -> Self {
        Self {
            id,
            top,
            bottom,
            right,
            left,
        }
    }

    fn assign_edges(&mut self, chars: &[char]) -> Self {
        Self {
            id: self.id,
            top: (chars[0] != '_').then(|| Edge::new(Dir::Up, chars[0])),
            bottom: (chars[1] != '_').then(|| Edge::new(Dir::Down, chars[1])),
            right: (chars[2] != '_').then(|| Edge::new(Dir::Right, chars[2])),
            left: (chars[3] != '_').then(|| Edge::new(Dir::Left, chars[3])),
        }
    }
}

#[derive(PartialEq)]
struct Tilemap {
    rows: u32,
    cols: u32,
    data: Vec<Vec<Option<Tile>>>,
}

impl Tilemap {
    fn new(rows: u32, cols: u32, tile_data: String) -> Self {
        // Tile is always ordered as North, South, East, West. (never eat soggy waffles)
        // string of a tile: abcd = {top: a, bottom: b, right: c, left: d}
        // string of a row: _bc_, efgh, ijkl, _, mnop. (the _ denotes an empty space or an edge
        let mut data = Vec::<Vec<Option<Tile>>>::new();
        let mut rng = thread_rng();
        tile_data.split('\n').for_each(|col| {
            let mut row = Vec::<Option<Tile>>::new();
            col.split(',')
                .map(|item: &str| item.trim())
                .for_each(|item: &str| {
                    if item == "_" {
                        row.push(None);
                        return;
                    }
                    let mut tile = Tile {
                        id: rng.next_u32(),
                        ..Default::default()
                    };
                    let chars: Vec<char> = item.chars().collect();
                    assert!(
                        chars.len() == 4,
                        "string of invalid length parsed: {item:#?}"
                    );

                    row.push(Some(tile.assign_edges(&chars)));
                });
            data.push(row);
        });

        Self { rows, cols, data }
    }

    fn solve(&self) -> Vec<Move> {
        let mut solution: Vec<Move> = Vec::<Move>::new();
        while !self.is_valid() {
            for (i, row) in self.data.iter().enumerate() {
                for (j, item) in row.iter().enumerate() {}
            }
        }

        solution
    }

    fn is_valid(&self) -> bool {
        for (i, row) in self.data.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                let Some(x) = tile else { continue };
                if i == 0 {
                    if j == 0 {
                        let right = self.right_validate(i, j, x);
                        let down = self.down_validate(i, j, x);
                        if !right || !down {
                            return false;
                        }
                    } else if j == self.data[i].len() {
                        let left = self.left_validate(i, j, x);
                        let down = self.down_validate(i, j, x);
                        if !left || !down {
                            return false;
                        }
                    } else if j > 0 && j < self.data[i].len() {
                        let left = self.left_validate(i, j, x);
                        let right = self.right_validate(i, j, x);
                        let down = self.down_validate(i, j, x);
                        if !left || !right || !down {
                            return false;
                        }
                    }
                } else if i == self.data.len() {
                    if j == 0 {
                        let right = self.right_validate(i, j, x);
                        let up = self.up_validate(i, j, x);
                        if !right || !up {
                            return false;
                        }
                    } else if j == self.data[i].len() {
                        let left = self.left_validate(i, j, x);
                        let up = self.up_validate(i, j, x);
                        if !left || !up {
                            return false;
                        }
                    } else if j > 0 && j < self.data[i].len() {
                        let left = self.left_validate(i, j, x);
                        let right = self.right_validate(i, j, x);
                        let up = self.up_validate(i, j, x);
                        if !left || !right || !up {
                            return false;
                        }
                    }
                } else if (i > 0 && i < self.data.len()) && j == 0 {
                    let right = self.right_validate(i, j, x);
                    let up = self.up_validate(i, j, x);
                    let down = self.down_validate(i, j, x);
                    if !right || !up || !down {
                        return false;
                    }
                } else if (i > 0 && i < self.data.len()) && j == self.data[i].len() {
                    let left = self.left_validate(i, j, x);
                    let up = self.up_validate(i, j, x);
                    let down = self.down_validate(i, j, x);
                    if !left || !up || !down {
                        return false;
                    }
                } else {
                    let left = self.left_validate(i, j, x);
                    let up = self.up_validate(i, j, x);
                    let right = self.right_validate(i, j, x);
                    let down = self.down_validate(i, j, x);
                    if !left || !up || !right || !down {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn up_validate(&self, i: usize, j: usize, x: &Tile) -> bool {
        match &self.data[i - 1][j] {
            Some(y) => {
                println!("{:#?} == {:#?}", x.top, y.bottom);
                x.top == y.bottom
            }
            None => true,
        }
    }

    fn left_validate(&self, i: usize, j: usize, x: &Tile) -> bool {
        match &self.data[i][j - 1] {
            Some(y) => {
                println!("{:#?} == {:#?}", x.left, y.right);
                x.left == y.right
            }
            None => true,
        }
    }

    fn down_validate(&self, i: usize, j: usize, x: &Tile) -> bool {
        match &self.data[i + 1][j] {
            Some(y) => {
                println!("{:#?} == {:#?}", x.bottom, y.top);
                x.bottom == y.top
            }
            None => true,
        }
    }

    fn right_validate(&self, i: usize, j: usize, x: &Tile) -> bool {
        match &self.data[i][j + 1] {
            Some(y) => {
                println!("{:#?} == {:#?}", x.right, y.left);
                x.right == y.left
            }
            None => true,
        }
    }
}

struct Move {
    tile: Tile,
    dir: Dir,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {}

    #[test]
    fn test_parser() {
        let to_parse: String = "_bc_, _, _de_".to_string();
        let tilemap: Tilemap = Tilemap::new(3, 1, to_parse);
        assert!(
            tilemap.data.len() == 1,
            "Tilemap didn't have expected data. Expected 1, got {}",
            tilemap.data.len()
        );

        assert!(
            tilemap.data[0].len() == 3,
            "First row of tilemap didn't ahve expected data. Expected 3, got {}",
            tilemap.data[0].len()
        );

        assert!(
            tilemap.data[0][0].is_some(),
            "first value of tilemap was None"
        );
        assert!(
            tilemap.data[0][1].is_none(),
            "second value of tilemap was Some"
        );
        assert!(
            tilemap.data[0][2].is_some(),
            "third value of tilemap was None"
        );

        let expected: Vec<Vec<Option<Tile>>> = vec![vec![
            Some(Tile::new(
                tilemap.data[0][0].clone().unwrap().id,
                None,
                Some(Edge::new(Dir::Down, 'b')),
                Some(Edge::new(Dir::Right, 'c')),
                None,
            )),
            None,
            Some(Tile::new(
                tilemap.data[0][2].clone().unwrap().id,
                None,
                Some(Edge::new(Dir::Down, 'd')),
                Some(Edge::new(Dir::Right, 'e')),
                None,
            )),
        ]];

        tilemap.data.into_iter().enumerate().for_each(|(i, row)| {
            row.into_iter()
                .enumerate()
                .for_each(|(j, item): (usize, Option<Tile>)| {
                    if item.is_none() {
                        assert!(expected[i][j].is_none(), "Value should have been None")
                    } else {
                        assert!(
                            item.clone().unwrap() == expected[i][j].clone().unwrap(),
                            "Inequivalent values!\n\nItem:\n{:#?}\n\nExpected:\n{:#?}\n",
                            item.unwrap(),
                            expected[i][j].clone().unwrap()
                        );
                    }
                })
        });
    }

    #[test]
    fn test_is_valid() {
        let to_parse: String =
            "_as_, _bts, _cut, _dvu, _e_v\nafw_, bgxw, chyx, dizy, ej_z\nf_o_, g_po, h_qp, i_rq, j__r"
                .to_string();
        let tilemap = Tilemap::new(3, 5, to_parse);
        assert!(tilemap.is_valid(), "Invalid tilemap");
    }
}
