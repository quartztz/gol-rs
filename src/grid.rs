#![allow(non_snake_case)]

extern crate rand;

use std::cmp; // for min/max

// traits

pub trait Stringable {
	fn to_string(&self) -> String;
}

pub trait Drawable {
	fn draw(&self, window: &pancurses::Window) -> bool;
}

// Cell

pub struct Cell {
	y: i32, 
	x: i32,
	alive: bool,
	sprite: char,
}

pub fn build_cell_from_prob(in_y: i32, in_x: i32) -> Cell {
	let al = rand::random(); 
	let sp = if al { 'O' } else { ' ' };
	Cell {
		y: in_y + 1, 
		x: in_x + 1, 
		alive: al,
		sprite: sp,
	}
}

pub fn build_cell(in_y: i32, in_x: i32, al: bool) -> Cell {
	let sp = if al { 'O' } else { ' ' };
	Cell {
		y: in_y,
		x: in_x,
		alive: al,
		sprite: sp,
	}
}

impl Cell {
	pub fn get_pos(&self) -> Pos {
		Pos {
			y: self.y, 
			x: self.x,
		}
	}

	pub fn get_alive(&self) -> bool {
		return self.alive;
	}
}

impl Stringable for Cell {
	fn to_string(&self) -> String {
		let al = if self.alive {"alive"} else {"dead"};
		return format!("this is a cell @ ({0}, {1}). it is {2}. \n", self.y, self.x, al);
	}
}

impl Drawable for Cell {
	fn draw(&self, window: &pancurses::Window) -> bool {
		window.mvaddch(self.y, self.x, self.sprite);
		return true;
	}
}

// Position

pub struct Pos {
	y: i32, 
	x: i32,
}

impl Pos {
	pub fn to_tuple(&self) -> (i32, i32) {
		return (self.y, self.x);
	}
}

pub fn pos_from_tuple(tup: (i32, i32)) -> Pos {
	Pos {
		y: tup.0,
		x: tup.1,
	}
}

// Grid

pub struct Grid {
	cells: Vec<Vec<Cell>>,
	max_y: i32, 
	max_x: i32,
}

pub fn build_grid(max_y: i32, max_x: i32, grid: Vec<Vec<Cell>>) -> Grid {
	Grid {
		cells: grid,
		max_y: max_y,
		max_x: max_x,
	}
	
}

pub fn build_grid_from_scratch(max_y: i32, max_x: i32) -> Grid {
	let mut line: Vec<Cell>; 
	let mut grid: Vec<Vec<Cell>> = vec![];
	for j in 0..(max_y - 1) {
		line = vec![]; 
		for i in 0..(max_x - 1) {
			line.push(build_cell_from_prob(j, i));
		}
		grid.push(line);
	}
	return build_grid(max_y, max_x, grid);
}

impl Grid {
	pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
		return &self.cells;
	}

	pub fn set_cells(&mut self, new_cells: Vec<Vec<Cell>>) -> () {
		self.cells = new_cells;
	}
	
	pub fn get_cell_at_pos(&self, pos: &Pos) -> &Cell {
		let line = self.cells.get(pos.y as usize).unwrap();
		return line.get(pos.x as usize).unwrap();
	}

	pub fn update(&mut self) -> () {

		let mut new_cells: Vec<Vec<Cell>> = vec![];
		for j in 0..self.cells.len() {
			let mut new_cells_line: Vec<Cell> = vec![];
			for i in 0..self.cells.get(0).unwrap().len() {
				let pos = Pos{y: j as i32, x: i as i32};
				let cell = self.get_cell_at_pos(&pos);
				let neighbours = self.get_alive_neighbours(&pos);
				let alive: bool;
				if cell.alive {
					if neighbours < 2 || neighbours > 3 { alive = false; } 
					else { alive = true; }
				} else {
					if neighbours == 3 { alive = true; } 
					else { alive = false; }
				}
				new_cells_line.push(build_cell(pos.y, pos.x, alive));
			}
			new_cells.push(new_cells_line);
		}
		self.set_cells(new_cells);
	}

	pub fn get_alive_neighbours(&self, pos: &Pos) -> i32 {
		return self.get_neighbours(pos).iter().sum();
	}

	pub fn get_neighbours(&self, pos: &Pos) -> Vec<i32> {
		let minY = cmp::max(pos.y - 1, 0);
		let maxY = cmp::min(pos.y + 1, self.max_y - 2);
		let minX = cmp::max(pos.x - 1, 0);
		let maxX = cmp::min(pos.x + 1, self.max_x - 2);

		let neighbours: Vec<Pos> = vec![
			Pos{y: pos.y, x: minX},
			Pos{y: maxY, x: minX},
			Pos{y: maxY, x: pos.x},
			Pos{y: maxY, x: maxX},
			Pos{y: pos.y, x: maxX},
			Pos{y: minY, x: maxX}, 
			Pos{y: minY, x: pos.x},
			Pos{y: minY, x: minX},
		];
		let mut ret: Vec<i32> = vec![];

		for pos in neighbours {
			if self.cells[pos.y as usize][pos.x as usize].get_alive() {
				ret.push(1); 
			} else {
				ret.push(0);
			}
		}
		
		return ret;	
	}
}

// implementations

impl Stringable for Grid {
	fn to_string(&self) -> String {
		let ret = format!("this is a {0} by {1} grid.", self.cells.len(), self.cells[0].len()) + 
							&format!("it contains the following {}", 1);
		return ret;
	}
}

impl Drawable for Grid {
	fn draw(&self, window: &pancurses::Window) -> bool {
		for line in &self.cells {
			for cell in line {
				cell.draw(window);
			}
		}
		return true;
	}
}