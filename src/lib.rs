extern crate wasm_bindgen;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[wasm_bindgen]
pub struct Field {
    width: usize,
    walls: Vec<usize>,
    start_idx: usize,
    end_idx: usize,
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
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn walls(&self) -> Vec<usize> {
        self.walls.clone()
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
                if current_idx < self.width {
                    return 0;
                } else {
                    return current_idx - self.width;
                }
            }
            Direction::Down => {
                if current_idx > self.width * self.width - 1 - self.width {
                    return 0;
                } else {
                    return current_idx + self.width;
                }
            }
        }
    }

    fn check_cell_makable(&self, walls: Vec<usize>, next_cell_idx: usize) -> bool {
        let mut count = 0;
        for i in 0..4 {
            let direction = Field::num_to_direction(i);
            let next_cell = self.get_next_cell(next_cell_idx, direction);
            if next_cell == 0 {
                continue;
            }
            if walls[next_cell] == 0 {
                count += 1;
            }
        }
        if count == 1 {
            return true;
        } else {
            return false;
        }
    }

    pub fn generate_maze(&self) {
        let mut walls = vec![1; self.width * self.width];
        walls[self.start_idx] = 0;
        let mut current = self.start_idx;
        let mut try_count: usize = 0;

        while current != self.end_idx {
            let next_dir = Field::num_to_direction(rnd(4));
            let next_cell_idx = self.get_next_cell(current, next_dir);
            if try_count > 5 {
                current = self.start_idx;
                walls = vec![1; self.width * self.width];
                continue;
            }
            if next_cell_idx != 0 && Field::check_cell_makable(self, walls, next_cell_idx) {
                walls[next_cell_idx] = 0;
                current = next_cell_idx;
                try_count = 0;
            } else {
                try_count += 1;
                continue;
            }
        }

        self.walls = vec![walls]
    }
