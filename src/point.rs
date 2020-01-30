use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}
impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }
    pub fn new_rand(
        x_min: u8,
        x_max: u8,
        y_min: u8,
        y_max: u8,
        occupied_points: &Vec<Point>,
    ) -> Point {
        let mut rng = thread_rng();

        let new_position = Point::new(rng.gen_range(x_min, x_max), rng.gen_range(y_min, y_max));
        if occupied_points.iter().any(|point| new_position == *point) {
            return Point::new_rand(x_min, x_max, y_min, y_max, occupied_points);
        } else {
            return new_position;
        }
    }
}
