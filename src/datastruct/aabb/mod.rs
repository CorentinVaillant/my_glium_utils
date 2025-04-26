#[cfg(test)]
mod test;

use num::Float;

use super::points::{As2dPoint, Point};

#[derive(Debug, Clone, Copy)]
pub struct Aabb<F: Float + Copy> {
    pub center: Point<F>,
    pub half_dim: F,
}

#[derive(Clone, Copy, Debug)]
pub enum DiagonalDirection {
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

impl<F: Float + Copy> Aabb<F> {
    pub fn new(center: (F, F), half_width: F) -> Self {
        debug_assert!(half_width > F::zero(), "half width should always be > 0.");
        Self {
            center: center.as_point(),
            half_dim: half_width,
        }
    }

    pub fn tchebychev_dist(self, point: Point<F>) -> F {
        let dx = (point.x - self.center.x).abs();
        let dy = (point.y - self.center.y).abs();
        dx.max(dy)
    }

    #[inline(always)]
    pub fn contain_pt(self, point: Point<F>) -> bool {
        self.tchebychev_dist(point) <= self.half_dim
    }

    pub fn intersect(self, other: Self) -> bool {
        self.tchebychev_dist(other.center) < self.half_dim + other.half_dim
    }

    #[inline(always)]
    pub fn diag_pos_from_center(&self, point: Point<F>) -> DiagonalDirection {
        match (point.x > self.center.x, point.y > self.center.y) {
            (false, false) => DiagonalDirection::DownLeft,
            (false, true) => DiagonalDirection::UpLeft,
            (true, false) => DiagonalDirection::DownRight,
            (true, true) => DiagonalDirection::UpRight,
        }
    }

    pub fn subdivide(self) -> [Self; 4] {
        let two = F::one() + F::one();
        let min_one = -F::one();
        let one = F::one();
        let quart_dim = self.half_dim / two;
        let offsets = [
            (min_one, one),
            (one, one),
            (one, min_one),
            (min_one, min_one),
        ];

        offsets.map(|(dx, dy)| Self {
            center: (
                self.center.x + dx * quart_dim,
                self.center.y + dy * quart_dim,
            )
                .as_point(),
            half_dim: quart_dim,
        })
    }

    pub fn from_min_max<T: As2dPoint<F>, U: As2dPoint<F>>(min: T, max: U) -> Self {
        let min = (min).as_point();
        let max = (max).as_point();
        let two = F::one() + F::one();

        let center = ((max.x + min.x) / (two), (max.y + min.y) / two).as_point();
        let half_dim = min.tchebychev_dist(max) / two;

        Self { center, half_dim }
    }
}

// impl Aabb<f32>{
//     #[inline]
//     fn contain_pt_simd(&self, x:f32,y:f32)->bool{
//         #[cfg(target_arch = "x86_64")]
//         if is_x86_feature_detected!("sse4.1"){
//             unsafe {
//                 let pt = _mm_set_ps(0.0, 0.0, y, x);
//                 let min = _mm_set_ps(0.0, 0.0, self.center.y - self.half_dim, self.center.x - self.half_dim);
//                 let max = _mm_set_ps(0.0, 0.0, self.center.y + self.half_dim, self.center.x + self.half_dim);

//                 let gt_min = _mm_cmpge_ps(pt, min);
//                 let lt_max = _mm_cmpge_ps(pt, max);
//                 let mask = _mm_movemask_ps(_mm_and_ps(gt_min, lt_max));

//                 (mask & 0b0011) == 0b0011
//             }

//         }else{
//             self.contain_pt((x,y).as_point())
//         }
//     }
// }
