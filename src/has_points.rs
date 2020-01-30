use crate::point::Point;

pub trait HasPoints {
    fn get_points(&self) -> &Vec<Point>;

    fn is_occupying_coordinates(&self, x: u8, y: u8) -> bool {
        let point = Point::new(x, y);
        self.is_occupying_point(point)
    }
    fn is_occupying_point(&self, point: Point) -> bool {
        self.get_points().iter().any(|e| *e == point)
    }
}
