use crate::board::Board;
use crate::game_cycle::GameCycle;
use crate::terminal::Terminal;
use pancurses::Input;
use std::time::Instant;

const ESC_KEY: char = '\u{001B}';
const FRAME_TIME: u128 = 800;

pub struct GameCycleController {
    terminal: Terminal,
    board: Board,
    has_first_input_happened: bool,
    is_game_running: bool,
}
impl GameCycleController {
    pub fn new(board: Board, terminal: Terminal) -> GameCycleController {
        GameCycleController {
            terminal,
            board,
            has_first_input_happened: false,
            is_game_running: true,
        }
    }
    pub fn run_input_logic(&mut self) -> Result<(), ()> {
        match self.terminal.get_input() {
            Some(Input::Character(c)) if c == ESC_KEY => Err(()),
            Some(input) => {
                self.board.on_input(input);
                self.has_first_input_happened = true;
                if self.is_game_running {
                    Ok(())
                } else {
                    Err(())
                }
            }
            _ => Ok(()),
        }
    }
    pub fn run_running_game_frame(&mut self) -> Result<(), ()> {
        match self.board.on_frame(()) {
            Ok(_) => match self.terminal.on_frame((&self.board,)) {
                Ok(_) => Ok(()),
                Err(_) => Ok(()),
            },
            Err(_) => {
                self.board.hide_snake = !self.board.hide_snake;
                self.terminal.on_game_over(&self.board);
                self.is_game_running = false;
                Err(())
            }
        }
    }
    pub fn run_ended_game_frame(&mut self) {
        self.board.hide_snake = !self.board.hide_snake;
        self.terminal.on_game_over(&self.board);
    }
    pub fn start(&mut self) {
        let start_instant = Instant::now();
        let mut prev_frame_delta_time = 0;

        self.terminal.setup();
        self.terminal.draw(&self.board);

        loop {
            let delta_time = start_instant.elapsed().as_millis();

            match self.run_input_logic() {
                Ok(_) => (),
                Err(_) => break,
            }

            let is_time_for_frame =
                prev_frame_delta_time == 0 || delta_time - prev_frame_delta_time >= FRAME_TIME;

            if is_time_for_frame && self.has_first_input_happened {
                prev_frame_delta_time = delta_time;

                if self.is_game_running {
                    match self.run_running_game_frame() {
                        Ok(_) => (),
                        Err(()) => (),
                    }
                } else {
                    self.run_ended_game_frame();
                }
            }
        }

        self.terminal.end();
    }
}
