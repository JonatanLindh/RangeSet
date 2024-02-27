use std::iter::{self, Sum};

use num_traits::Num;
use std::hash::Hash;

use crate::{range_set::RangeSet, Point, Rangelike};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Rectangle<T: Num + Copy> {
    top: T,
    left: T,
    bottom: T,
    right: T,
}

impl<T: Num + Ord + Copy> Rectangle<T> {
    pub fn from_tlbr(top: T, left: T, bottom: T, right: T) -> Option<Self> {
        if top <= bottom || right <= left {
            return None;
        }

        Some(Self {
            top,
            left,
            bottom,
            right,
        })
    }

    pub fn new(c1: Point<T>, c2: Point<T>) -> Self {
        let top = c1.y.max(c1.y);
        let left = c1.x.min(c1.x);
        let bottom = c1.y.min(c1.y);
        let right = c1.x.max(c1.x);

        Self {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl<T: Num + Ord + Copy + Hash + Sum> Rangelike for Rectangle<T> {
    type Inner = Self;
    type Unit = T;

    fn is_empty(&self) -> bool {
        (self.top == self.bottom) || (self.right == self.left)
    }

    /// Area for rects
    fn count(&self) -> T {
        (self.top - self.bottom) * (self.right - self.left)
    }

    fn contains(&self, other: &Self) -> bool {
        self.top >= other.top
            && self.left <= other.left
            && self.bottom <= other.bottom
            && self.right >= other.right
    }

    fn intersects(&self, other: &Self) -> bool {
        if self.right <= other.left {
            return false;
        }

        if self.left >= other.right {
            return false;
        }

        if self.bottom >= other.top {
            return false;
        }

        if self.top <= other.bottom {
            return false;
        }

        true
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        Rectangle::from_tlbr(
            self.top.min(other.top),
            self.left.max(other.left),
            self.bottom.max(other.bottom),
            self.right.min(other.right),
        )
    }

    fn difference(&self, other: &Self) -> RangeSet<Self> {
        if !self.intersects(other) {
            return RangeSet::from_iter(iter::once(self.clone()));
        }

        if other.contains(self) {
            return RangeSet::new();
        }

        let t =
            Rectangle::from_tlbr(self.top, self.left, other.top, self.right);

        let b = Rectangle::from_tlbr(
            other.bottom,
            self.left,
            self.bottom,
            self.right,
        );

        let l = Rectangle::from_tlbr(
            other.top.min(self.top),
            self.left,
            other.bottom.max(self.bottom),
            other.left,
        );

        let r = Rectangle::from_tlbr(
            other.top.min(self.top),
            other.right,
            other.bottom.max(self.bottom),
            self.right,
        );

        [t, l, b, r]
            .into_iter()
            .flatten()
            .collect()
    }

    fn union(&self, other: &Self) -> RangeSet<Self> {
        iter::once(self.clone())
            .chain(
                other
                    .difference(self)
                    .into_iter()
                    .cloned(),
            )
            .collect()
    }

    fn symmetric_difference(&self, other: &Self) -> RangeSet<Self> {
        todo!()
    }
}
