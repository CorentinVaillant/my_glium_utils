#![cfg(test)]

use crate::datastruct::quadtree::{Aabb, As2dPoint, Quadtree};

#[derive(Debug, Clone)]
struct TestPoint {
    x: f32,
    y: f32,
}

impl As2dPoint<f32> for TestPoint {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
}

#[test]
fn test_quadtree_insert() {
    let boundary = Aabb::new((5.0, 5.0), 5.0);
    let mut quadtree: Quadtree<f32, TestPoint, 4> = Quadtree::empty(boundary);

    let point = TestPoint { x: 6.0, y: 6.0 };
    assert!(quadtree.insert(point).is_ok());
}

#[test]
fn test_quadtree_query_range() {
    let boundary = Aabb::new((5.0, 5.0), 5.0);
    let mut quadtree: Quadtree<f32, TestPoint, 4> = Quadtree::empty(boundary);

    let point = TestPoint { x: 4.0, y: 4.0 };
    quadtree.insert(point).unwrap();

    let query_range = Aabb::new((4.0, 4.0), 1.0);
    let result = quadtree.query_range(query_range);

    assert_eq!(result.len(), 1);
}

#[test]
fn test_rebuild() {
    let boundary = Aabb::new((0., 0.), 100.);
    let mut qtree: Quadtree<f32, TestPoint, 3> = Quadtree::empty(boundary);

    let points = vec![
        TestPoint { x: 4.0, y: 4.0 },
        TestPoint { x: 8.0, y: 4.0 },
        TestPoint { x: 10.0, y: 5.0 },
        TestPoint { x: 25.0, y: -1.0 },
    ];
    for p in points {
        qtree.insert(p).unwrap();
    }
}

#[test]
#[ignore]
fn test_lot_of_insert() {
    #[derive(Debug, Clone)]
    struct TestPoint64 {
        x: f64,
        y: f64,
    }

    impl As2dPoint<f64> for TestPoint64 {
        fn x(&self) -> f64 {
            self.x
        }
        fn y(&self) -> f64 {
            self.y
        }
    }

    // on my pc -> 5.78s
    const POINT_NB: u32 = 10_000_000;
    let half_dim = POINT_NB as f64;

    let boundary = Aabb::new((0., 0.), half_dim + 500.);
    let mut qtree: Quadtree<f64, TestPoint64, 3> = Quadtree::empty(boundary);
    let delta = (POINT_NB as f64).recip();
    let mut i_f = 0.;
    for _ in 0..POINT_NB {
        i_f += delta;
        let p = TestPoint64 {
            x: f64::cos(half_dim * i_f) * i_f * half_dim,
            y: f64::sin(half_dim * i_f) * i_f * half_dim,
        };
        qtree.insert(p).unwrap();
    }
}

#[test]
#[ignore]
fn test_lot_of_map_then_map_with_elem_in_range_then_map() {
    #[derive(Debug, Clone)]
    struct TestPoint64 {
        x: f64,
        y: f64,
    }

    impl As2dPoint<f64> for TestPoint64 {
        fn x(&self) -> f64 {
            self.x
        }
        fn y(&self) -> f64 {
            self.y
        }
    }

    const POINT_NB: u32 = 10_000_000;
    let half_dim = POINT_NB as f64;

    let boundary = Aabb::new((0., 0.), half_dim + 500.);
    let delta = (POINT_NB as f64).recip();
    let mut i_f = 0.;
    let points = (0..POINT_NB)
        .map(|_| {
            i_f += delta;
            TestPoint64 {
                x: f64::cos(half_dim * i_f) * i_f * half_dim,
                y: f64::sin(half_dim * i_f) * i_f * half_dim,
            }
        })
        .collect();

    let mut qtree: Quadtree<f64, TestPoint64, 3> = Quadtree::new(boundary, points);
    println!("all points in !");

    let range_mapping = |p: &TestPoint64| Aabb::new((p.x, p.y), 100.);
    let first_map = |p: &mut TestPoint64| p.x += 1.;
    let map_with_other = |p1: &mut TestPoint64, p2: &mut TestPoint64| {
        p1.x += p2.x / 1000.;
        p1.y += p2.y / 1000.;

        p2.x += p1.x / 1000.;
        p2.y += p1.y / 1000.;
    };

    let last_map = |p: &mut TestPoint64| p.y += 1.;

    let begin = std::time::Instant::now();
    qtree.map_then_map_with_elem_in_range_then_map(
        first_map,
        range_mapping,
        map_with_other,
        last_map,
    );
    let time_pass = std::time::Instant::now()
        .duration_since(begin)
        .as_secs_f64();
    println!(
        "Ok -> took {} for Quadtree::map_then_map_with_elem_in_range_then_map",
        time_pass
    );
}
