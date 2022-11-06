extern crate wasm_bindgen;
extern crate wee_alloc;
// extern crate rand;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
// use rand::Rng;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    a + b
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
    pub fn new(width: usize ) -> Field {
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

    pub fn generate_maze(&self) -> Vec<usize> {
        let mut walls = self.walls.clone();
        let mut rng = rand::thread_rng();
        let mut stack = Vec::new();
        let mut current = self.start_idx;
        let mut visited = vec![false; self.width * self.width];
        visited[current] = true;
        walls
    }
}

// #[wasm_bindgen(module = "/www/utils/rnd.js")]
// extern {
// 		fn rnd(max: usize) -> usize;
// }

// #[wasm_bindgen]
// #[derive(PartialEq)]
// pub enum Direction {
// 		Up,
// 		Down,
// 		Left,
// 		Right,
// }

// #[wasm_bindgen]
// #[derive(Clone, Copy)]
// pub enum GameStatus {
// 		Won,
// 		Lost,
// 		Played,
// }

// #[derive(Clone, Copy, PartialEq)]
// pub struct SnakeCell(usize);
// struct Snake {
// 	body: Vec<SnakeCell>,
// 	direction: Direction,
// }


// impl Snake {
// 	fn new(spaws_index: usize, size: usize) -> Snake {
// 		let mut body = vec!();

// 		for i in 0..size {
// 			body.push(SnakeCell(spaws_index - i));
// 		}

// 		Snake {
// 			body: body,
// 			direction: Direction::Up,
// 		}
// 	}
// }







// #[wasm_bindgen]
// impl World {
// 		pub fn new(width: usize, snake_idx: usize) -> World {

// 				let snake = Snake::new(snake_idx, 3);
// 				let size = width * width;

// 				World {
// 						width,
// 						size,
// 						reward_cell: World::gen_reward_cell(size, &snake.body),
// 						snake,
// 						next_cell: None,
// 						status: None,
// 						points: 0,
// 				}
// 		}

// 		fn gen_reward_cell(max:usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
// 			let mut reward_cell;

// 			loop {
// 				reward_cell = rnd(max);
// 				if !snake_body.contains(&SnakeCell(reward_cell)) {
// 					break;
// 				}
// 			}

// 			Some(reward_cell)
// 		}

// 		pub fn width(&self) -> usize {
// 				self.width
// 		}

// 		pub fn points(&self) -> usize {
// 				self.points
// 		}

// 		pub fn reward_cell(&self) -> Option<usize> {
// 				self.reward_cell
// 		}

// 		pub fn snake_head_idx(&self) -> usize {
// 				self.snake.body[0].0
// 		}

// 		pub fn start_game(&mut self) {
// 				self.status = Some(GameStatus::Played);
// 		}

// 		pub fn game_status(&self) -> Option<GameStatus> {
// 				self.status
// 		}

// 		pub fn game_status_text(&self) -> String {
// 				match self.status {
// 						Some(GameStatus::Won) => "You won!".to_string(),
// 						Some(GameStatus::Lost) => "You lost!".to_string(),
// 						Some(GameStatus::Played) => "Playing".to_string(),
// 						None => String::from("No status")
// 				}
// 		}

// 		pub fn change_snake_dir(&mut self, direction: Direction) {
// 				let next_cell = self.gen_next_snake_cell(&direction);

// 				if self.snake.body[1].0 == next_cell.0 { return; }

// 				self.next_cell = Some(next_cell);
// 				self.snake.direction = direction;
// 		}

// 		pub fn snake_length(&self) -> usize {
// 				self.snake.body.len()
// 		}

// 		// snake.bodyの最初の要素のポインタを返す
// 		pub fn snake_cells(&self) ->  *const SnakeCell {
// 				// as_ptrはheapのアドレス（&はスタックのアドレス参照になる）
// 				self.snake.body.as_ptr()
// 		}

// 		pub fn step(&mut self) {

// 				match self.status {
// 						Some(GameStatus::Played) => {
// 							let temp = self.snake.body.clone();

// 							match self.next_cell {
// 									Some(cell) => {
// 											self.snake.body[0] = cell;
// 											self.next_cell = None;
// 									},
// 									None => {
// 											self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
// 									},
// 							}

// 							let len = self.snake.body.len();

// 							for i in 1..len {
// 									self.snake.body[i] = SnakeCell(temp[i - 1].0);
// 							}

// 							if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
// 									self.status = Some(GameStatus::Lost)
// 							}

// 							if self.reward_cell == Some(self.snake_head_idx()) {
// 									if self.snake_length() < self.size {
// 										self.points += 1;
// 										self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
// 									} else {
// 										self.reward_cell = None;
// 										self.status = Some(GameStatus::Won)
// 									}
// 									self.snake.body.push(SnakeCell(self.snake.body[1].0));
// 							}
// 						},
// 						_ => {},
// 				}
// 		}

// 		fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
// 			let snake_idx = self.snake_head_idx();
// 			let row = snake_idx / self.width;

// 			return match direction {
// 				Direction::Right => {
// 					let threshold = (row + 1) * self.width;
// 					if (snake_idx + 1) == threshold {
// 						SnakeCell(threshold - self.width)
// 					} else {
// 						SnakeCell(snake_idx + 1)
// 					}
// 				},
// 				Direction::Left => {
// 					let threshold = row * self.width;
// 					if snake_idx == threshold {
// 						SnakeCell(threshold + self.width - 1)
// 					} else {
// 						SnakeCell(snake_idx - 1)
// 					}
// 				},
// 				Direction::Up => {
// 					let threshold = snake_idx - (row * self.width);
// 					if snake_idx == threshold {
// 						SnakeCell((self.size - self.width) + threshold)
// 					} else {
// 						SnakeCell(snake_idx - self.width)
// 					}
// 				},
// 				Direction::Down => {
// 					let threshold = snake_idx + ((self.width - row) * self.width);
// 					if snake_idx + self.width == threshold {
// 						SnakeCell(threshold - ((row + 1 ) * self.width))
// 					} else {
// 						SnakeCell(snake_idx + self.width)
// 					}
// 				},
// 			};
// 		}


// }
