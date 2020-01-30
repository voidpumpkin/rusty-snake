use crate::food::Food;
use crate::game_cycle::GameCycle;
use crate::has_points::HasPoints;
use crate::point::Point;
use crate::snake::Snake;
use pancurses::Input;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub width: u8,  //including walls
    pub height: u8, //including walls
    pub snake: Snake,
    pub food: Food,
    pub hide_snake: bool,
}

type BoardProps = ();

impl GameCycle<BoardProps> for Board {
    fn on_frame(&mut self, _props: BoardProps) -> Result<(), ()> {
        let food_points = self.food.get_points();
        let snake_head_point = self.snake.get_head_point();
        let wall_points = self.get_wall_points();

        self.snake.on_frame((&food_points, &wall_points))?;
        self.food.on_frame((
            snake_head_point,
            self.width,
            self.height,
            self.snake.get_points(),
        ))?;
        Ok(())
    }
    fn on_input(&mut self, input: Input) {
        self.snake.on_input(input);
        self.food.on_input(input);
    }
}

impl Board {
    pub fn new(width: u8, height: u8, snake: Snake, food: Food) -> Board {
        Board {
            width,
            height,
            snake,
            food,
            hide_snake: false,
        }
    }
    pub fn get_wall_points(&self) -> Vec<Point> {
        let mut walls: Vec<Point> = vec![];
        for i in 0..self.width {
            walls.push(Point::new(i, 0));
            walls.push(Point::new(i, self.height));
        }
        for i in 0..self.height {
            walls.push(Point::new(0, i));
            walls.push(Point::new(self.width, i));
        }
        walls
    }
}

/*
* Desired output
* ┏━━━━━━━━━━━┓
* ┃▒          ┃
* ┃▒          ┃
* ┃▒▒▒█   +   ┃
* ┗━━━━━━━━━━━┛
*/

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let get_char = |x, y| {
            let is_snake = self.snake.is_occupying_coordinates(x, y);
            let is_snake_head = self.snake.is_head_at_coordinates(x, y);
            let is_food = self.food.is_occupying_coordinates(x, y);

            let is_left_border = x == 0;
            let is_right_border = x == self.width - 1;
            let is_bottom_border = y == 0;
            let is_top_border = y == self.height - 1;

            let is_top_left_corner = is_top_border && is_left_border;
            let is_top_right_corner = is_top_border && is_right_border;
            let is_bottom_left_corner = is_bottom_border && is_left_border;
            let is_bottom_right_corner = is_bottom_border && is_right_border;

            match true {
                _v if !self.hide_snake && _v == is_snake_head => '█',
                _v if !self.hide_snake && _v == is_snake => '▒',
                _v if _v == is_food => '+',
                _v if _v == is_top_left_corner => '┏',
                _v if _v == is_top_right_corner => '┓',
                _v if _v == is_bottom_left_corner => '┗',
                _v if _v == is_bottom_right_corner => '┛',
                _v if _v == (is_left_border || is_right_border) => '┃',
                _v if _v == (is_top_border || is_bottom_border) => '━',
                _ => ' ',
            }
        };

        let get_row = |y| -> String {
            let mut row = String::new();
            for x in 0..self.width {
                row.push(get_char(x, y))
            }
            row
        };

        for y in (0..self.height).rev() {
            writeln!(f, "{}", get_row(y))?;
        }
        Ok(())
    }
}
