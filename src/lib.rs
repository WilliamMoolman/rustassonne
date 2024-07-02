#![allow(dead_code)]

use std::{
    collections::VecDeque,
    fmt::{self, Debug, Formatter},
};
use wasm_bindgen::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum EdgeType {
    Grass,
    Road,
    Intersection,
    City,
    CityPlus,
    Monastary,
}

use EdgeType::*;

impl EdgeType {
    fn char(&self) -> char {
        match self {
            Grass => 'O',
            Road => 'R',
            Intersection => 'X',
            City => 'C',
            CityPlus => '#',
            Monastary => '+',
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Tile(EdgeType, EdgeType, EdgeType, EdgeType, EdgeType);

impl Tile {
    fn left(&self) -> EdgeType {
        self.0
    }
    fn up(&self) -> EdgeType {
        self.1
    }
    fn down(&self) -> EdgeType {
        self.2
    }
    fn right(&self) -> EdgeType {
        self.3
    }
    fn centre(&self) -> EdgeType {
        self.4
    }

    fn up_left(&self) -> EdgeType {
        if self.centre() != City && self.centre() != CityPlus {
            return Grass;
        }
        if self.up() == City && self.left() == City {
            return City;
        };

        Grass
    }

    fn down_left(&self) -> EdgeType {
        if self.centre() != City && self.centre() != CityPlus {
            return Grass;
        }
        if self.down() == City && self.left() == City {
            return City;
        };

        Grass
    }

    fn up_right(&self) -> EdgeType {
        if self.centre() != City && self.centre() != CityPlus {
            return Grass;
        }
        if self.up() == City && self.right() == City {
            return City;
        };

        Grass
    }

    fn down_right(&self) -> EdgeType {
        if self.centre() != City && self.centre() != CityPlus {
            return Grass;
        }
        if self.down() == City && self.right() == City {
            return City;
        };

        Grass
    }
}

impl Tile {
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;
    const D: usize = 3;
    const E: usize = 4;
    const F: usize = 5;
    const G: usize = 6;
    const H: usize = 7;
    const I: usize = 8;
    const J: usize = 9;
    const K: usize = 10;
    const L: usize = 11;
    const M: usize = 12;
    const N: usize = 13;
    const O: usize = 14;
    const P: usize = 15;
    const Q: usize = 16;
    const R: usize = 17;
    const S: usize = 18;
    const T: usize = 19;
    const U: usize = 20;
    const V: usize = 21;
    const W: usize = 22;
    const X: usize = 23;

    const STANDARD: [Tile; 24] = [
        Tile(Grass, Grass, Grass, Road, Monastary),
        Tile(Grass, Grass, Grass, Grass, Monastary),
        Tile(City, City, City, City, CityPlus),
        Tile(Road, City, Road, Grass, Road),
        Tile(Grass, City, Grass, Grass, Grass),
        Tile(City, Grass, City, Grass, CityPlus),
        Tile(City, Grass, City, Grass, City),
        Tile(City, Grass, City, Grass, Grass),
        Tile(Grass, City, City, Grass, Grass),
        Tile(Grass, City, Road, Road, Road),
        Tile(Road, City, Grass, Road, Road),
        Tile(Road, City, Road, Road, Intersection),
        Tile(Grass, City, City, Grass, CityPlus),
        Tile(Grass, City, City, Grass, City),
        Tile(City, City, Road, Road, CityPlus),
        Tile(City, City, Road, Road, City),
        Tile(City, City, City, Grass, CityPlus),
        Tile(City, City, City, Grass, City),
        Tile(City, City, City, Road, CityPlus),
        Tile(City, City, City, Road, City),
        Tile(Grass, Road, Grass, Road, Road),
        Tile(Road, Grass, Grass, Road, Road),
        Tile(Road, Grass, Road, Road, Intersection),
        Tile(Road, Road, Road, Road, Intersection),
    ];
    const EMPTY: usize = 255;
    const CLICKABLE: usize = 254;
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(
            f,
            "{}{}{}",
            self.up_left().char(),
            self.up().char(),
            self.up_right().char()
        )?;
        writeln!(
            f,
            "{}{}{}",
            self.left().char(),
            self.centre().char(),
            self.right().char()
        )?;
        write!(
            f,
            "{}{}{}",
            self.down_left().char(),
            self.down().char(),
            self.down_right().char()
        )
    }
}
#[derive(Clone)]
struct TilePlacement {
    tile: usize,
    rotation: u8,
}

impl TilePlacement {
    fn new(tile: usize, rotation: u8) -> TilePlacement {
        TilePlacement { tile, rotation }
    }
    fn from(tile: usize) -> TilePlacement {
        TilePlacement { tile, rotation: 0 }
    }
}

#[wasm_bindgen]
pub struct Game {
    tiles_remaining: [usize; 24],
    pile: Vec<usize>,
    board: VecDeque<VecDeque<TilePlacement>>,
}

#[wasm_bindgen]
impl Game {
    pub fn place_next(&mut self, position_idx: u8, rotation: u8) -> bool {
        if let Some(tile) = self.pile.pop() {
            let row = position_idx / self.width();
            let col = position_idx % self.width();
            return self.place_tile(tile as u8, row, col, rotation);
        }
        false
    }
    fn place_tile(&mut self, tile_id: u8, row: u8, col: u8, rotation: u8) -> bool {
        let mut row = row;
        let mut col = col;
        if row == 0 {
            let width = self.width() as usize;
            self.board.push_front(VecDeque::from(vec![
                TilePlacement::from(Tile::EMPTY);
                width
            ]));
            row += 1;
        }
        if row == self.height() - 1 {
            let width = self.width() as usize;
            self.board.push_back(VecDeque::from(vec![
                TilePlacement::from(Tile::EMPTY);
                width
            ]));
        }

        if col == 0 {
            for row in self.board.iter_mut() {
                row.push_front(TilePlacement::from(Tile::EMPTY));
            }
            col += 1;
        }
        if col == self.width() - 1 {
            for row in self.board.iter_mut() {
                row.push_back(TilePlacement::from(Tile::EMPTY));
            }
        }

        // Do some error checking

        self.board[row as usize][col as usize] = TilePlacement::new(tile_id as usize, rotation);
        self.tiles_remaining[tile_id as usize] -= 1;

        for r in 0..self.height() as usize {
            for c in 0..self.width() as usize {
                if self.board[r][c].tile != Tile::EMPTY { continue; }
                let mut clickable = false;
                if r == 0 {
                    if self.board[r+1][c].tile != Tile::CLICKABLE && self.board[r+1][c].tile != Tile::EMPTY { clickable = true; }
                } else if r == self.height() as usize - 1 {
                    if self.board[r-1][c].tile != Tile::CLICKABLE && self.board[r-1][c].tile != Tile::EMPTY { clickable = true; }
                } else {
                    if self.board[r+1][c].tile != Tile::CLICKABLE && self.board[r+1][c].tile != Tile::EMPTY { clickable = true; }
                    if self.board[r-1][c].tile != Tile::CLICKABLE && self.board[r-1][c].tile != Tile::EMPTY { clickable = true; }
                }

                if c == 0 {
                    if self.board[r][c+1].tile != Tile::CLICKABLE && self.board[r][c+1].tile != Tile::EMPTY { clickable = true; }
                } else if c == self.width() as usize - 1 {
                    if self.board[r][c-1].tile != Tile::CLICKABLE && self.board[r][c-1].tile != Tile::EMPTY { clickable = true; }
                } else {
                    if self.board[r][c+1].tile != Tile::CLICKABLE && self.board[r][c+1].tile != Tile::EMPTY { clickable = true; }
                    if self.board[r][c-1].tile != Tile::CLICKABLE && self.board[r][c-1].tile != Tile::EMPTY { clickable = true; }
                }

                if clickable {
                    self.board[r][c].tile = Tile::CLICKABLE;
                }
            }
        }

        true
    }
    //
    //#[wasm_bindgen(constructor)]
    pub fn standard() -> Game {
        let board = VecDeque::from(vec![
            VecDeque::from(vec![
                TilePlacement::from(Tile::EMPTY),
                TilePlacement::from(Tile::CLICKABLE),
                TilePlacement::from(Tile::EMPTY),
            ]),
            VecDeque::from(vec![
                TilePlacement::from(Tile::CLICKABLE),
                TilePlacement::new(Tile::D, 0),
                TilePlacement::from(Tile::CLICKABLE),
            ]),
            VecDeque::from(vec![
                TilePlacement::from(Tile::EMPTY),
                TilePlacement::from(Tile::CLICKABLE),
                TilePlacement::from(Tile::EMPTY),
            ]),
        ]);

        let tiles_remaining = [
            2, 4, 1, 3, 5, 2, 1, 3, 2, 3, 3, 3, 2, 3, 2, 3, 1, 3, 2, 1, 8, 9, 4, 1,
        ];

        let mut pieces = vec![];
        for (idx, &number) in tiles_remaining.iter().enumerate() {
            for _ in 0..number {
                pieces.push(idx);
            }
        }
        pieces.shuffle(&mut thread_rng());


        Game {
            tiles_remaining,
            pile: pieces,
            board,
        }
    }

    pub fn get_remaining(&self) -> Vec<u8> {
        let mut remaining = [0; 24];
        for (i, left) in self.tiles_remaining.iter().enumerate() {
            remaining[i] = *left as u8;
        }
        remaining.to_vec()
    }

    pub fn width(&self) -> u8 {
        self.board[0].len() as u8
    }

    pub fn height(&self) -> u8 {
        self.board.len() as u8
    }

    pub fn tiles(&self) -> Vec<u8> {
        let mut tile_ids = vec![];
        for row in &self.board {
            for col in row {
                tile_ids.push(col.tile as u8);
            }
        }

        tile_ids
    }

    pub fn next_tile(&self) -> u8 {
        self.pile[self.pile.len()-1] as u8
    }

    pub fn tiles_rotation(&self) -> Vec<u8> {
        let mut rotations = vec![];
        for row in &self.board {
            for col in row {
                rotations.push(col.rotation as u8);
            }
        }

        rotations
    }
}
