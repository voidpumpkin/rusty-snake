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
    pub fn start(&mut self) {
        let GameCycleController {
            terminal, board, ..
        } = self;
        let start_instant = Instant::now();
        let mut prev_frame_time = 0;

        terminal.setup();
        terminal.draw(&board);

        loop {
            match terminal.get_input() {
                Some(Input::Character(c)) if c == ESC_KEY => break,
                Some(input) => {
                    board.on_input(input);
                    self.has_first_input_happened = true;
                    if !self.is_game_running {
                        break;
                    }
                }
                _ => (),
            };

            let run_time = start_instant.elapsed().as_millis();
            let is_time_for_frame =
                prev_frame_time == 0 || run_time - prev_frame_time >= FRAME_TIME;
            if is_time_for_frame && self.has_first_input_happened {
                prev_frame_time = run_time;
                if self.is_game_running {
                    match board.on_frame(()) {
                        Ok(_) => match terminal.on_frame((&board,)) {
                            _ => (),
                        },
                        Err(_) => {
                            board.hide_snake = !board.hide_snake;
                            terminal.on_game_over(&board);
                            self.is_game_running = false;
                        }
                    }
                } else {
                    board.hide_snake = !board.hide_snake;
                    terminal.on_game_over(&board);
                }
            }
        }

        terminal.end();
    }
}
