use crate::maths::types::Vec2;

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

    pub fn dist_sq(self, other:Self)->f32{
        let dx_sq = (other.x - self.x) * (other.x - self.x);
        let dy_sq = (other.y - self.y) * (other.y - self.y);

        dx_sq + dy_sq
    }

    #[inline]
    pub fn dist(self, other: Self)->f32{
        self.dist_sq(other).sqrt()
    }

    pub fn tchebychev_dist(self, other: Self) -> f32 {
        let dx = (other.x - self.x).abs();
        let dy = (other.y - self.y).abs();
        dx.max(dy)
    }
}



impl<T:As2dPoint> From<T> for Point{
    fn from(value: T) -> Self {
        Self { x: value.x(), y: value.y() }
    }
}

impl As2dPoint for (f32,f32){
    #[inline]
    fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    fn y(&self) -> f32 {
        self.1
    }
}

impl As2dPoint for [f32;2]{
    #[inline]
    fn x(&self) -> f32 {
        self[0]
    }

    #[inline]
    fn y(&self) -> f32 {
        self[1]
    }
}

impl As2dPoint for Vec2 {
    #[inline]
    fn x(&self) -> f32 {
        self[0]
    }

    #[inline]
    fn y(&self) -> f32 {
        self[1]
    }
}

impl As2dPoint for IndexPoint{
    #[inline]
    fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    fn y(&self) -> f32 {
        self.y
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
