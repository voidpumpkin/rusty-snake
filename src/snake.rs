use crate::direction::Direction;
use crate::game_cycle::GameCycle;
use crate::has_points::HasPoints;
use crate::point::Point;
use pancurses::Input;

#[derive(Debug)]
pub struct Snake {
    pub points: Vec<Point>,
    pub facing_direction: Direction,
    input_dir: Direction,
}
type SnakeProps<'a, 'b> = (&'a Vec<Point>, &'b Vec<Point>);

impl GameCycle<SnakeProps<'_, '_>> for Snake {
    fn on_frame(&mut self, props: SnakeProps) -> Result<(), ()> {
        let (food_points, wall_points) = props;

        self.turn(self.input_dir);
        match self.get_head_point() {
            Some(head) => {
                if food_points.iter().any(|point| *point == head) {
                    self.mov_eat(wall_points)?;
                } else {
                    self.mov(wall_points)?;
                }
            }
            None => self.mov(wall_points)?,
        };
        Ok(())
    }
    fn on_input(&mut self, input: Input) {
        self.input_dir = match input {
            Input::KeyUp => Direction::Up,
            Input::KeyRight => Direction::Right,
            Input::KeyDown => Direction::Down,
            Input::KeyLeft => Direction::Left,
            _ => self.input_dir,
        };
    }
}

impl Snake {
    pub fn new(facing_direction: Direction, points: Vec<Point>) -> Snake {
        Snake {
            points,
            facing_direction,
            input_dir: facing_direction,
        }
    }
    pub fn get_head_point(&self) -> Option<Point> {
        if self.points.len() != 0 {
            Some(self.points[0])
        } else {
            None
        }
    }
    pub fn is_head_at_coordinates(&self, x: u8, y: u8) -> bool {
        match self.get_head_point() {
            Some(point) => point == Point::new(x, y),
            None => false,
        }
    }
    pub fn mov(&mut self, wall_points: &Vec<Point>) -> Result<(), ()> {
        let poped = self.points.pop();
        match self.mov_eat(wall_points) {
            Ok(()) => Ok(()),
            Err(err) => match poped {
                Some(poped) => {
                    self.points.push(poped);
                    Err(err)
                }
                None => Err(err),
            },
        }
    }
    pub fn mov_eat(&mut self, wall_points: &Vec<Point>) -> Result<(), ()> {
        use Direction::{Down, Left, Right, Up};
        let Point { x, y } = self.points[0];
        let new_position = match self.facing_direction {
            Up => Point::new(x, y + 1),
            Right => Point::new(x + 1, y),
            Down => Point::new(x, y - 1),
            Left => Point::new(x - 1, y),
        };
        let has_hit_wall = wall_points.iter().any(|&point| new_position == point);
        let has_hit_self = self.points.iter().any(|&point| new_position == point);

        if has_hit_wall || has_hit_self {
            return Err(());
        }
        self.points.insert(0, new_position);
        Ok(())
    }
    pub fn turn(&mut self, dir: Direction) {
        if dir != self.facing_direction.opposite() {
            self.facing_direction = dir;
        }
    }
}

impl HasPoints for Snake {
    fn get_points(&self) -> &Vec<Point> {
        &self.points
    }
}
