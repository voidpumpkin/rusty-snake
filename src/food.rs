use crate::game_cycle::GameCycle;
use crate::has_points::HasPoints;
use crate::point::Point;
use pancurses::Input;

#[derive(Debug)]
pub struct Food {
    points: Vec<Point>,
}

type FoodProps<'a> = (Option<Point>, u8, u8, &'a Vec<Point>);

impl GameCycle<FoodProps<'_>> for Food {
    fn on_frame(&mut self, props: FoodProps) -> Result<(), ()> {
        let (snake_head_opt, board_width, board_height, snake_points) = props;
        match snake_head_opt {
            Some(snake_head) => {
                if snake_head == self.points[0] {
                    self.on_eaten(board_width, board_height, snake_points);
                }
            }
            None => (),
        }
        Ok(())
    }
    fn on_input(&mut self, _input: Input) {}
}

impl Food {
    pub fn new(points: Vec<Point>) -> Food {
        Food { points }
    }
    pub fn on_eaten(&mut self, board_width: u8, board_height: u8, snake_points: &Vec<Point>) {
        self.points.pop();
        self.points.push(Point::new_rand(
            1,
            board_width - 1,
            1,
            board_height - 1,
            snake_points,
        ));
    }
}

impl HasPoints for Food {
    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}
