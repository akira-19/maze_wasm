extern crate wasm_bindgen;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum GameStatus {
    BeforePlaying,
    Playing,
    Done,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Field {
    width: usize,
    walls: Vec<usize>,
    start_idx: usize,
    end_idx: usize,
    player_idx: usize,
    status: GameStatus,
}

#[wasm_bindgen]
impl Field {
    pub fn new(width: usize) -> Field {
        let mut vec = vec![0; width * width];
        for i in 0..(vec.len()) {
            if i < width || i % width == 0 || i % width == width - 1 || i > vec.len() - width {
                vec[i] = 1;
            }
        }
        Field {
            width,
            walls: vec,
            start_idx: width + 1,
            end_idx: width * width - width - 2,
            player_idx: width + 1,
            status: GameStatus::BeforePlaying,
        }
    }

    fn get_initial_available_cells(&self) -> Vec<usize> {
        let mut available_cells = Vec::new();
        for i in 0..(self.width * self.width) {
            if i < self.width
                || i % self.width == 0
                || i % self.width == self.width - 1
                || i > self.width * self.width - self.width
            {
                continue;
            }
            available_cells.push(i);
        }
        available_cells
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn walls(&self) -> Vec<usize> {
        self.walls.clone()
    }

    pub fn player_idx(&self) -> usize {
        self.player_idx
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn move_player(&mut self, direction: Direction) {
        if self.status != GameStatus::Playing {
            return;
        }
        let mut new_idx = self.player_idx;
        match direction {
            Direction::Up => {
                new_idx -= self.width;
            }
            Direction::Down => {
                new_idx += self.width;
            }
            Direction::Left => {
                new_idx -= 1;
            }
            Direction::Right => {
                new_idx += 1;
            }
        }
        if self.walls[new_idx] == 0 {
            self.player_idx = new_idx;
        }
        if self.player_idx == self.end_idx {
            self.status = GameStatus::Done;
        }
    }

    fn num_to_direction(num: usize) -> Direction {
        match num {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction number"),
        }
    }

    fn get_next_cell(&self, current_idx: usize, direction: Direction) -> (usize) {
        match direction {
            Direction::Right => {
                let threshold = self.width - 2;
                let col = current_idx % self.width;
                if col == threshold {
                    return 0;
                } else {
                    return current_idx + 1;
                }
            }
            Direction::Left => {
                let threshold = 1;
                let col = current_idx % self.width;
                if col == threshold {
                    return 0;
                } else {
                    return current_idx - 1;
                }
            }
            Direction::Up => {
                if current_idx < self.width * 2 - 1 {
                    return 0;
                } else {
                    return current_idx - self.width;
                }
            }
            Direction::Down => {
                if current_idx > self.width * self.width - 1 - 2 * self.width {
                    return 0;
                } else {
                    return current_idx + self.width;
                }
            }
        }
    }

    fn get_reachable_directions(&self, walls: &Vec<usize>, current_idx: usize) -> Vec<Direction> {
        let mut directions = Vec::new();
        for i in 0..4 {
            let direction = Field::num_to_direction(i);
            let d = direction.clone();
            let next_cell = self.get_next_cell(current_idx, d);
            if next_cell != 0 {
                let mut counter = 0;
                for i in 0..4 {
                    let dir = Field::num_to_direction(i);
                    let after_next_cell = self.get_next_cell(next_cell, dir);
                    if after_next_cell != 0 && walls[after_next_cell] == 0 {
                        counter += 1;
                    }
                }
                if counter == 1 {
                    directions.push(direction);
                }
            }
        }
        directions
    }

    pub fn generate_maze(&mut self) {
        if self.status != GameStatus::BeforePlaying {
            return;
        }
        self.status = GameStatus::Playing;
        let mut walls = vec![1; self.width * self.width];
        walls[self.start_idx] = 0;
        let mut current = self.start_idx;

        while current != self.end_idx {
            let reachable_directions = self.get_reachable_directions(&walls, current);

            if reachable_directions.len() == 0 {
                current = self.start_idx;
                walls = vec![1; self.width * self.width];
                walls[self.start_idx] = 0;
                continue;
            }
            let next_dir = reachable_directions[rnd(reachable_directions.len())];
            let next_cell_idx = self.get_next_cell(current, next_dir);

            walls[next_cell_idx] = 0;
            current = next_cell_idx;
        }

        let mut digable_cells = self.serch_digable_cells(&walls);
        while digable_cells.len() > 0 {
            let idx = rnd(digable_cells.len());
            let cell = digable_cells[idx];
            walls[cell] = 0;
            self.dig_aisle(cell);
            digable_cells = self.serch_digable_cells(&walls);
        }

        self.walls = walls;
    }

    fn is_cell_digable(&self, walls: &Vec<usize>, current_idx: usize) -> bool {
        let mut counter = 0;
        if walls[current_idx] == 0 {
            return false;
        }
        for i in 0..4 {
            let direction = Field::num_to_direction(i);
            let next_cell = self.get_next_cell(current_idx, direction);
            if walls[next_cell] == 0 {
                counter += 1;
            }
        }
        counter == 1
    }

    fn serch_digable_cells(&self, walls: &Vec<usize>) -> Vec<usize> {
        let mut digable_cells = Vec::new();
        let cells = self.get_initial_available_cells();
        for cell_idx in cells {
            if self.is_cell_digable(walls, cell_idx) {
                digable_cells.push(cell_idx);
            }
        }
        digable_cells
    }

    fn dig_aisle(&mut self, start_idx: usize) {
        self.walls[start_idx] = 0;
        let mut current = start_idx;

        while true {
            let reachable_directions = self.get_reachable_directions(&self.walls, current);

            if reachable_directions.len() == 0 {
                break;
            }
            let next_dir = reachable_directions[rnd(reachable_directions.len())];
            let next_cell_idx = self.get_next_cell(current, next_dir);

            self.walls[next_cell_idx] = 0;
            current = next_cell_idx;
        }
    }
}
