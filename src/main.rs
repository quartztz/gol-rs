#![allow(non_snake_case)]

pub mod grid;

use grid::*;
use pancurses::*;
use std::time::Duration;
use std::thread::sleep;

static SLEEP_TIME: Duration = Duration::from_millis(300);

enum State {
	INIT(),
	SIMULATION(),
	PAUSED(),
	END(),
}

fn update_simulation(win: &Window, grid: &mut Grid) -> State {
	win.clear();
	grid.update();
	grid.draw(win);

	sleep(SLEEP_TIME);

	match win.getch() {
		Some(Input::Character(c)) => { 
			if c == 'q' { 
				State::END()
			} else if c == '\n' {
				State::PAUSED()
			}else {
				State::SIMULATION()
			}},
		Some(_input) => { State::SIMULATION() },
		None => ( State::SIMULATION() ),
	}
}

fn init(win: &Window) -> State {
	// TODO: state setup using mouse before starting the simulation.
	match win.getch() {
		Some(Input::Character(c)) => {
			if c == 's' {
				State::SIMULATION()
			} else {
				State::INIT() 
			}
		}
		Some(_input) => State::INIT(),
		None => State::INIT(),
	}
}

fn paused(win: &Window) -> State {
	match win.getch() {
		Some(Input::Character(c)) => {
			if c == '\n' {
				State::SIMULATION()
			} else {
				State::PAUSED()
			}
		}
		Some(_input) => State::PAUSED(),
		None => State::PAUSED()
	}
}

fn main() {
	let main_win = initscr(); 
	curs_set(0);
	noecho();
	main_win.nodelay(true);
	let (max_y, max_x) = main_win.get_max_yx();

	let mut grid = build_grid_from_scratch(max_y, max_x);
	let mut current_state: State = State::INIT();

	'main: loop {
		match current_state {
			State::INIT() => { current_state = init(&main_win) },
			State::SIMULATION() => { current_state = update_simulation(&main_win, &mut grid) }
			State::PAUSED() => { current_state = paused(&main_win) }
			State::END() => { break 'main; }
		}
	}

	endwin();
}