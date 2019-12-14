#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate point;
pub use point::Point2D;

pub mod circle;
pub use circle::Circle;

pub enum Shape2D {
    POINT(Point2D<f32>),
    CIRCLE(Circle<f32>),
    RECT(),
    POLIGON(),
    LINE()
}