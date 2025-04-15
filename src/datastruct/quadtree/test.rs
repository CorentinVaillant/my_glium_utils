#![cfg(test)]

use crate::datastruct::quadtree::{Aabb, As2dPoint, Quadtree};



#[derive(Debug, Clone)]
struct TestPoint {
    x: f32,
    y: f32,
}

impl As2dPoint for TestPoint {
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
    let mut quadtree: Quadtree<TestPoint, 4> = Quadtree::empty(boundary);

    let point = TestPoint { x: 6.0, y: 6.0 };
    assert!(quadtree.insert(point).is_ok());
}

#[test]
fn test_quadtree_query_range() {
    let boundary = Aabb::new((5.0, 5.0), 5.0);
    let mut quadtree: Quadtree<TestPoint, 4> = Quadtree::empty(boundary);

    let point = TestPoint { x: 4.0, y: 4.0 };
    quadtree.insert(point).unwrap();

    let query_range = Aabb::new((4.0, 4.0), 1.0);
    let result = quadtree.query_range(query_range);

    assert_eq!(result.len(), 1);
}

#[test]
fn test_rebuild() {
    let boundary = Aabb::new((0., 0.), 100.);
    let mut qtree: Quadtree<TestPoint, 3> = Quadtree::empty(boundary);

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
    // on my pc -> 11.39s
    const POINT_NB: u32 = 10_000_000;
    let half_dim = POINT_NB as f32;

    let boundary = Aabb::new((0., 0.), half_dim + 500.);
    let mut qtree: Quadtree<TestPoint, 3> = Quadtree::empty(boundary);
    for i in 0..POINT_NB {
        println!("iter : {i}");
        let i_f = half_dim - i as f32 / 2.;
        let p = TestPoint {
            x: i_f,
            y: f32::sin(i as f32),
        };
        qtree.insert(p).unwrap();
    }
}
