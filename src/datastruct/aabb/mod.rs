#[cfg(test)]
mod test;

use super::points::{As2dPoint, Point};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub center: Point,
    pub half_dim: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum DiagonalDirection {
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

impl Aabb {
    pub fn new(center: (f32, f32), half_width: f32) -> Self {
        debug_assert!(half_width > 0., "half width should always be > 0.");
        Self {
            center: center.into(),
            half_dim: half_width,
        }
    }

    pub fn tchebychev_dist(self, point: Point) -> f32 {
        let dx = (point.x - self.center.x).abs();
        let dy = (point.y - self.center.y).abs();
        dx.max(dy)
    }

    #[inline]
    pub fn contain_pt(self, point: Point) -> bool {
        self.tchebychev_dist(point) <= self.half_dim
    }

    pub fn intersect(self, other: Self) -> bool {
        self.tchebychev_dist(other.center) < self.half_dim + other.half_dim
    }

    pub fn diag_pos_from_center(&self, point: Point) -> DiagonalDirection {
        match (point.x > self.center.x, point.y > self.center.y) {
            (false, false) => DiagonalDirection::DownLeft,
            (false, true) => DiagonalDirection::UpLeft,
            (true, false) => DiagonalDirection::DownRight,
            (true, true) => DiagonalDirection::UpRight,
        }
    }

    pub fn subdivide(self) -> [Self; 4] {
        let quart_dim = self.half_dim / 2.;
        let offsets = [(-1., 1.), (1., 1.), (1., -1.), (-1., -1.)];

        offsets.map(|(dx, dy)| Self {
            center: (
                self.center.x + dx * quart_dim,
                self.center.y + dy * quart_dim,
            )
                .into(),
            half_dim: quart_dim,
        })
    }

    pub fn from_min_max<T: As2dPoint, U: As2dPoint>(min: T,max:U)->Self{
        let min = Point::from(min);
        let max = Point::from(max);

        let center = ((max.x + min.x)/2., (max.y + min.y)/2.).into();
        let half_dim = min.tchebychev_dist(max) /2.;

        Self { center, half_dim }
    }
}
