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
    pub fn new_random(min: Point, max: Point, occupied_points: &Vec<Point>) -> Point {
        let mut rng = thread_rng();

        let new_position = Point::new(rng.gen_range(min.x, max.x), rng.gen_range(min.y, max.y));
        if occupied_points.iter().any(|point| new_position == *point) {
            return Point::new_random(min, max, occupied_points);
        } else {
            return new_position;
        }
    }
}
