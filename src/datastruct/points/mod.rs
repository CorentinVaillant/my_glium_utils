use my_rust_matrix_lib::my_matrix_lib::prelude::VectorMath;
use num::Float;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy)]
pub struct Point<F: Float + Copy> {
    pub x: F,
    pub y: F,
}

impl<F: Float + Copy> Point<F> {
    ///false if either x or y are NaN or Infinite
    #[inline(always)]
    pub fn as_valid_coord(&self) -> bool {
        !(self.x.is_nan() || self.y.is_nan() || self.x.is_infinite() || self.y.is_infinite())
    }

    #[inline(always)]
    pub fn dist_sq(self, other: Self) -> F {
        let dx_sq = (other.x - self.x) * (other.x - self.x);
        let dy_sq = (other.y - self.y) * (other.y - self.y);

        dx_sq + dy_sq
    }

    #[inline(always)]
    pub fn dist(self, other: Self) -> F {
        self.dist_sq(other).sqrt()
    }

    #[inline(always)]
    pub fn tchebychev_dist(self, other: Self) -> F {
        let dx = (other.x - self.x).abs();
        let dy = (other.y - self.y).abs();
        dx.max(dy)
    }
}

impl<F: Float + Copy> As2dPoint<F> for (F, F) {
    #[inline(always)]
    fn x(&self) -> F {
        self.0
    }

    #[inline(always)]
    fn y(&self) -> F {
        self.1
    }
}

impl<F: Float + Copy> As2dPoint<F> for [F; 2] {
    #[inline(always)]
    fn x(&self) -> F {
        self[0]
    }

    #[inline(always)]
    fn y(&self) -> F {
        self[1]
    }
}

impl<F: Float + Copy> As2dPoint<F> for VectorMath<F, 2> {
    #[inline(always)]
    fn x(&self) -> F {
        self[0]
    }

    #[inline(always)]
    fn y(&self) -> F {
        self[1]
    }
}

impl<F: Float + Copy> As2dPoint<F> for IndexPoint<F> {
    #[inline(always)]
    fn x(&self) -> F {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> F {
        self.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IndexPoint<F: Float + Copy> {
    pub x: F,
    pub y: F,
    pub i: usize,
}

impl<F: Float + Copy> IndexPoint<F> {
    #[inline(always)]
    pub fn new(x: F, y: F, i: usize) -> Self {
        Self { x, y, i }
    }

    #[inline(always)]
    pub fn into_point(self) -> Point<F> {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

pub trait As2dPoint<F: Float + Copy> {
    fn x(&self) -> F;
    fn y(&self) -> F;

    #[inline(always)]
    fn as_point(&self) -> Point<F> {
        Point {
            x: self.x(),
            y: self.y(),
        }
    }
}
