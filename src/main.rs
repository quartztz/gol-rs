#![allow(non_snake_case)]

pub mod grid;

use grid::*;
use pancurses::*;
use std::time::Duration;
use std::thread::sleep;

fn main() {
	let main_win = initscr(); 
	curs_set(0);
	noecho();
	main_win.nodelay(true);
	let (max_y, max_x) = main_win.get_max_yx();

	let sleep_time = Duration::from_millis(300);

	let mut grid = build_grid_from_scratch(max_y, max_x);

	'main: loop {
		main_win.clear();
		grid.update();
		grid.draw(&main_win);
	
		sleep(sleep_time);

		match main_win.getch() {
			Some(Input::Character(c)) => { if c == 'q' { break 'main; } },
			Some(_input) => {},
			None => (),
		} 
	}
	endwin();
}