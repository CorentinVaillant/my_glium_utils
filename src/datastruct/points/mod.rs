#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    ///false if either x or y are NaN or Infinite
    pub fn as_valid_coord(&self) -> bool {
        !(self.x.is_nan() || self.y.is_nan() || self.x.is_infinite() || self.y.is_infinite())
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (f32, f32) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IndexPoint {
    pub x: f32,
    pub y: f32,
    pub i: usize,
}

impl IndexPoint {
    pub fn new(x: f32, y: f32, i: usize) -> Self {
        Self { x, y, i }
    }

    pub fn into_point(self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

pub trait As2dPoint {
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn as_point(&self) -> Point {
        (self.x(), self.y()).into()
    }
}
