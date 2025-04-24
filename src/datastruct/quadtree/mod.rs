use std::{fmt::Debug, mem::MaybeUninit};

use arrayvec::ArrayVec;
use num::Float;

use super::{
    aabb::{Aabb, DiagonalDirection},
    points::{As2dPoint, IndexPoint},
};

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct Quadtree<F: Float + Copy + Debug, T: As2dPoint<F>, const N: usize> {
    vec: Vec<T>,
    base_node: Node<F, N>,
}

#[derive(Debug, Clone, Copy)]
pub enum QuadtreeError<F: Float + Copy + Debug> {
    OutOfBoundary(Aabb<F>, (F, F)),
    InvalidCoord((F, F)),
}

impl<F: Float + Copy + Debug + Debug> std::fmt::Display for QuadtreeError<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuadtreeError::OutOfBoundary(aabb, pt) => {
                write!(f, "{:?} does not contains the point {:?}.", aabb, pt)
            }
            QuadtreeError::InvalidCoord(coord) => {
                write!(f, "point of coord {:?} are invalid.", { coord })
            }
        }
    }
}

#[allow(dead_code)]
impl<F: Float + Copy + Debug, T: As2dPoint<F>, const N: usize> Quadtree<F, T, N> {
    pub fn empty(boundary: Aabb<F>) -> Self {
        debug_assert!(N > 0, "The size should be a least 1");

        Self {
            vec: vec![],
            base_node: Node::empty(boundary),
        }
    }

    pub fn new(boundary: Aabb<F>, vec: Vec<T>) -> Self {
        debug_assert!(N > 0, "The size should be a least 1");

        let mut result = Self {
            vec,
            base_node: Node::empty(boundary),
        };
        result.rebuild_fit();
        result
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn depth(&self) -> usize {
        self.base_node.depth()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.vec.iter_mut()
    }

    pub fn insert(&mut self, elem: T) -> Result<(), QuadtreeError<F>> {
        let i = self.vec.len();
        let i_p = IndexPoint::new(elem.x(), elem.y(), i);

        if !self.base_node.boundary.contain_pt(i_p.into_point()) {
            return Err(QuadtreeError::OutOfBoundary(
                self.base_node.boundary,
                (i_p.x, i_p.y),
            ));
        }

        self.vec.push(elem);

        self.base_node.insert(i_p)
          .unwrap_or_else(|_| panic!("something went wrong in QuadTree::insert: could not insert the value, even if it is in the Tree boundary ({:?}), the Tchebychev distance from the center is ({:?}), OOB : {}\n\t=>",
          self.base_node.boundary,self.base_node.boundary.tchebychev_dist(i_p.into_point()),self.base_node.boundary.tchebychev_dist(i_p.into_point()) > self.base_node.boundary.half_dim));

        Ok(())
    }

    pub fn insert_fit(&mut self, elem: T) {
        let i = self.vec.len();
        let i_p = IndexPoint {
            x: elem.x(),
            y: elem.y(),
            i,
        };

        self.vec.push(elem);

        if self.base_node.insert(i_p).is_err() {
            self.rebuild_fit();
        }
    }

    pub fn query_range(&self, range: Aabb<F>) -> Vec<&T> {
        let mut result = vec![];
        for i_p in self.base_node.query_range(range) {
            result.push(&self.vec[i_p]);
        }

        result
    }

    pub fn map_query_range(&mut self, range: Aabb<F>, map: impl Fn(&mut T)) {
        for i_point in self.base_node.query_range(range) {
            map(&mut self.vec[i_point]);
        }
    }

    pub fn map_with_elem_in_range(
        &mut self,
        range_mapping: impl Fn(&T) -> Aabb<F>,
        map: impl Fn(&mut T, &mut T),
    ) {
        for i in 0..self.vec.len() {
            let range = self.base_node.query_range(range_mapping(&self.vec[i]));

            for i_p in range {
                match i_p.cmp(&i) {
                    std::cmp::Ordering::Greater => {
                        let (split_i, split_p) = self.vec.split_at_mut(i_p);
                        map(&mut split_i[i], &mut split_p[0]);
                        // println!("2. \t=> searching next")
                    }
                    std::cmp::Ordering::Less => {
                        let (split_p, split_i) = self.vec.split_at_mut(i);
                        map(&mut split_p[i_p], &mut split_i[0]);
                        // println!("2. \t=> searching next")
                    }
                    _ => (),
                };
            }
        }
    }

    //Horible name
    ///For each point in the quadtree :
    /// 1. first_map(point)  
    /// 2. for each other in range_mapping(point) :  
    ///     ->  2.1 map_with_other(point,other)  
    /// 3. last_map(point)  
    pub fn map_then_map_with_elem_in_range_then_map(
        &mut self,
        first_map: impl Fn(&mut T),
        range_mapping: impl Fn(&T) -> Aabb<F>,
        map_with_other: impl Fn(&mut T, &mut T),
        last_map: impl Fn(&mut T),
    ) {
        let mut new_base_node = Node::empty(self.base_node.boundary);
        let mut failed_to_insert = false;

        for i in 0..self.vec.len() {
            let range = self.base_node.query_range(range_mapping(&self.vec[i]));

            first_map(&mut self.vec[i]);

            for i_p in range {
                if i_p > i {
                    let (left, right) = self.vec.split_at_mut(i_p);
                    map_with_other(&mut left[i], &mut right[0]);
                }
            }
            last_map(&mut self.vec[i]);

            if !failed_to_insert {
                let new_i_pt = IndexPoint {
                    x: self.vec[i].x(),
                    y: self.vec[i].y(),
                    i,
                };
                failed_to_insert = new_base_node.insert(new_i_pt).is_err();
            }
        }
        if !failed_to_insert {
            self.base_node = new_base_node;
        } else {
            #[cfg(debug_assertions)]
            eprintln!("rebuild the entiere tree");
            self.rebuild_fit();
        }
    }

    pub fn rebuild_fit(&mut self) {
        if !self
            .vec
            .iter()
            .all(|p| self.base_node.boundary.contain_pt(p.as_point()))
        {
            let (min_x, max_x, min_y, max_y) = self.vec.iter().fold(
                (
                    F::infinity(),
                    F::neg_infinity(),
                    F::infinity(),
                    F::neg_infinity(),
                ),
                |(min_x, max_x, min_y, max_y), elem| {
                    (
                        min_x.min(elem.x()) - F::one(),
                        max_x.max(elem.x()) + F::one(),
                        min_y.min(elem.y()) - F::one(),
                        max_y.max(elem.y()) + F::one(),
                    )
                },
            );

            let width = max_x - min_x;
            let height = max_y - min_y;
            let two = F::one() + F::one();
            let new_half_width = (width.max(height) / two).abs().max(F::epsilon());
            let new_center = ((min_x + max_x) / two, (min_y + max_y) / two);

            self.base_node = Node::empty(Aabb::new(new_center, new_half_width));
        } else {
            self.base_node = Node::empty(self.base_node.boundary);
        }

        for (i, elem) in self.vec.iter().enumerate() {
            let elem_pt = IndexPoint {
                x: elem.x(),
                y: elem.y(),
                i,
            };
            if let Err(e) = self.base_node.insert(elem_pt) {
                match e {
                    QuadtreeError::OutOfBoundary(_, _) => panic!(
                        "QuadTree::rebuild went wrong : All points should fit after resize\n\t=>{e:?}"
                    ),
                    QuadtreeError::InvalidCoord(_) => {
                        panic!(
                            "QuadTree::rebuild went wrong : elem: {i} does not have valid coordinate\n\t=>{e:?}"
                        )
                    }
                }
            }
        }
    }

    pub fn rebuild(&mut self) -> Result<(), QuadtreeError<F>> {
        let mut new_node = Node::empty(self.base_node.boundary);

        for (i, elem) in self.vec.iter().enumerate() {
            let i_pt = IndexPoint {
                x: elem.x(),
                y: elem.y(),
                i,
            };

            new_node.insert(i_pt)?;
        }
        self.base_node = new_node;
        Ok(())
    }

    pub fn change_bounds(&mut self, new_bound: Aabb<F>) -> Result<(), QuadtreeError<F>> {
        let mut new_node = Node::empty(new_bound);
        for (i, elem) in self.vec.iter().enumerate() {
            let elem_pt = IndexPoint {
                x: elem.x(),
                y: elem.y(),
                i,
            };

            new_node.insert(elem_pt)?;
        }

        self.base_node = new_node;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Node<F: Float + Copy + Debug, const N: usize> {
    boundary: Aabb<F>,
    data: NodeData<F, N>,
}

impl<F: Float + Copy + Debug, const N: usize> Node<F, N> {
    fn empty(boundary: Aabb<F>) -> Self {
        Self {
            boundary,
            data: NodeData::Leaf(NodeLeafData {
                points: ArrayVec::new(),
            }),
        }
    }

    fn insert(&mut self, p_i: IndexPoint<F>) -> Result<(), QuadtreeError<F>> {
        #[cfg(debug_assertions)]
        const MAX_LOOP: usize = 500_000;
        #[cfg(all(test, not(debug_assertions)))]
        const MAX_LOOP: usize = 200_000;

        #[cfg(debug_assertions)]
        let mut it_num = 0;

        let pt = p_i.into_point();

        if !pt.as_valid_coord() {
            return Err(QuadtreeError::InvalidCoord((p_i.x, p_i.y)));
        }
        let mut curr_data = &mut self.data;
        let mut curr_bounds = self.boundary;

        while curr_bounds.contain_pt(pt) {
            match curr_data {
                NodeData::Child(child) => {
                    let dir = curr_bounds.diag_pos_from_center(pt);
                    (curr_data, curr_bounds) = child.get_child_mut(dir)
                }
                NodeData::Leaf(leaf) => {
                    if leaf.points.is_full() {
                        curr_data.subdivide_into_child_data(curr_bounds);
                        continue;
                    } else {
                        leaf.points.push(p_i);
                        return Ok(());
                    }
                }
            }

            #[cfg(debug_assertions)]
            {
                it_num += 1;
                debug_assert!(it_num < MAX_LOOP, "to much iteration")
            }
        }
        Err(QuadtreeError::OutOfBoundary(curr_bounds, (p_i.x, p_i.y)))
    }

    fn depth(&self) -> usize {
        1 + match &self.data {
            NodeData::Child(node_child_data) => node_child_data
                .down_left
                .depth()
                .max(node_child_data.down_right.depth())
                .max(
                    node_child_data
                        .up_left
                        .depth()
                        .max(node_child_data.up_right.depth()),
                ),
            _ => 0,
        }
    }

    fn query_range(&self, range: Aabb<F>) -> Vec<usize> {
        //iterative

        let mut result = Vec::new();
        let mut stack = vec![self];

        while let Some(curr_node) = stack.pop() {
            if !curr_node.boundary.intersect(range) {
                continue;
            }
            match &curr_node.data {
                NodeData::Child(child) => {
                    stack.push(&child.up_right);
                    stack.push(&child.up_left);
                    stack.push(&child.down_right);
                    stack.push(&child.down_left);
                }
                NodeData::Leaf(leaf) => {
                    for i_p in &leaf.points {
                        result.push(i_p.i);
                    }
                }
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
enum NodeData<F: Float + Copy + Debug, const N: usize> {
    Child(NodeChildData<F, N>),
    Leaf(NodeLeafData<F, N>),
}

impl<F: Float + Copy + Debug, const N: usize> NodeData<F, N> {
    fn new_leaf(points: ArrayVec<IndexPoint<F>, N>) -> Self {
        Self::Leaf(NodeLeafData { points })
    }

    #[inline(always)]
    fn subdivide_into_child_data(&mut self, boundary: Aabb<F>) {
        //Should review this function
        let uninit: &mut MaybeUninit<NodeData<F, N>> = unsafe { core::mem::transmute(self) };
        let prev = core::mem::replace(uninit, MaybeUninit::uninit());

        let assumed_init = unsafe { prev.assume_init() };

        let new = match assumed_init {
            NodeData::Child(_) => assumed_init,
            NodeData::Leaf(leaf) => NodeData::Child(leaf.subdivide_into_child_data(boundary)),
        };

        let _ = core::mem::replace(uninit, MaybeUninit::new(new));
    }
}

#[derive(Debug, Clone)]
struct NodeChildData<F: Float + Copy + Debug, const N: usize> {
    up_right: Box<Node<F, N>>,
    up_left: Box<Node<F, N>>,

    down_left: Box<Node<F, N>>,
    down_right: Box<Node<F, N>>,
}

impl<F: Float + Copy + Debug, const N: usize> NodeChildData<F, N> {
    #[inline(always)]
    fn get_child_mut(&mut self, dir: DiagonalDirection) -> (&mut NodeData<F, N>, Aabb<F>) {
        match dir {
            DiagonalDirection::UpRight => (&mut self.up_right.data, self.up_right.boundary),
            DiagonalDirection::UpLeft => (&mut self.up_left.data, self.up_left.boundary),
            DiagonalDirection::DownLeft => (&mut self.down_left.data, self.down_left.boundary),
            DiagonalDirection::DownRight => (&mut self.down_right.data, self.down_right.boundary),
        }
    }
}

#[derive(Debug, Clone)]
struct NodeLeafData<F: Float + Copy + Debug, const N: usize> {
    points: ArrayVec<IndexPoint<F>, N>,
}

impl<F: Float + Copy + Debug, const N: usize> NodeLeafData<F, N> {
    fn subdivide_into_child_data(self, boundary: Aabb<F>) -> NodeChildData<F, N> {
        let [ul, ur, dr, dl] = boundary.subdivide();
        let [mut ur_p, mut ul_p, mut dl_p, mut dr_p] = [
            ArrayVec::new(),
            ArrayVec::new(),
            ArrayVec::new(),
            ArrayVec::new(),
        ];
        for p in &self.points {
            {
                match boundary.diag_pos_from_center(p.into_point()) {
                    DiagonalDirection::UpLeft => ul_p.push(*p),
                    DiagonalDirection::UpRight => ur_p.push(*p),
                    DiagonalDirection::DownRight => dr_p.push(*p),
                    DiagonalDirection::DownLeft => dl_p.push(*p),
                }
            }
        }

        let up_right = Box::new(Node {
            boundary: ur,
            data: NodeData::new_leaf(ur_p),
        });
        let up_left = Box::new(Node {
            boundary: ul,
            data: NodeData::new_leaf(ul_p),
        });
        let down_left = Box::new(Node {
            boundary: dl,
            data: NodeData::new_leaf(dl_p),
        });
        let down_right = Box::new(Node {
            boundary: dr,
            data: NodeData::new_leaf(dr_p),
        });

        NodeChildData {
            up_right,
            up_left,
            down_left,
            down_right,
        }
    }
}
