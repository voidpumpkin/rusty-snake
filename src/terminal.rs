use crate::board::Board;
use crate::game_cycle::GameCycle;
use crate::has_points::HasPoints;
use pancurses::{curs_set, endwin, noecho, Input, Window};

pub struct Terminal {
    window: Window,
}

type TerminalProps<'a> = (&'a Board,);

impl GameCycle<TerminalProps<'_>> for Terminal {
    fn on_frame(&mut self, props: TerminalProps) -> Result<(), ()> {
        let (board,) = props;
        self.window.clear();
        self.draw(&board);
        Ok(())
    }
    fn on_input(&mut self, _input: Input) {}
}

impl Terminal {
    pub fn new(window: Window) -> Terminal {
        Terminal { window }
    }
    pub fn setup(&self) {
        noecho();
        curs_set(0);
        self.window.refresh();
        self.window.keypad(true);
        self.window.nodelay(true);
    }
    pub fn draw(&self, board: &Board) {
        self.window.printw(format!("{}", board));
    }
    pub fn end(&self) {
        endwin();
    }
    pub fn get_input(&self) -> Option<Input> {
        self.window.getch()
    }
    pub fn on_game_over(&self, board: &Board) {
        self.window.clear();
        self.draw(board);
        self.window.printw(format!(
            "Game over, Snake length: {}",
            board.snake.get_points().len()
        ));
    }
}
