#![cfg(test)]

use crate::datastruct::{aabb::Aabb, points::Point};

#[test]
fn test_aabb_contain_point1() {
    let aabb = Aabb::new((5.0, 5.0), 3.0);
    let inside_point = Point { x: 6.0, y: 6.0 };
    let outside_point = Point { x: 9.0, y: 9.0 };

    assert!(aabb.contain_pt(inside_point));
    assert!(!aabb.contain_pt(outside_point));
}

#[test]
fn test_aabb_intersect() {
    let aabb1 = Aabb::new((5.0, 5.0), 3.0);
    let aabb2 = Aabb::new((6.0, 6.0), 3.0);
    let aabb3 = Aabb::new((10.0, 10.0), 2.0);

    assert!(aabb1.intersect(aabb2));
    assert!(!aabb1.intersect(aabb3));
}

#[test]
fn test_aabb_subdivide() {
    let aabb = Aabb::new((5.0, 5.0), 4.0);
    let quadrants = aabb.subdivide();

    assert_eq!(quadrants.len(), 4);
    assert_eq!(quadrants[0].half_dim, 2.0);
}