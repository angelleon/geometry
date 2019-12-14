use std::fmt::{Display, Formatter, Result};

pub struct Circle<T>
where T: Default + Display {
    pub x: T,
    pub y: T,
    pub r: T
}

impl<T> Circle<T>
where T: Default + Display {
    pub fn new(x: T, y: T, r: T) -> Self {
        Circle {
            x,
            y,
            r
        }
    }
}

impl<T> Display for Circle<T>
where T: Display + Default {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("x: {}, y: {}, r: {}", self.x, self.y, self.r)
        )
    }
}