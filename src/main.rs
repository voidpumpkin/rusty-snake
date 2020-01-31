extern crate pancurses;
extern crate rand;

mod board;
mod direction;
mod food;
mod game_cycle;
mod game_cycle_controller;
mod has_points;
mod point;
mod snake;
mod terminal;

use board::Board;
use direction::Direction;
use food::Food;
use game_cycle_controller::GameCycleController;
use pancurses::initscr;
use point::Point;
use snake::Snake;
use terminal::Terminal;

fn main() {
    let initial_snake = Snake::new(
        Direction::Right,
        vec![Point::new(3, 1), Point::new(2, 1), Point::new(1, 1)],
    );
    let initial_food = Food::new(vec![Point::new(4, 6)]);
    let board = Board::new(20, 10, initial_snake, initial_food);
    let terminal = Terminal::new(initscr());
    let mut game_cycle_controller = GameCycleController::new(board, terminal);

    game_cycle_controller.start();
}
