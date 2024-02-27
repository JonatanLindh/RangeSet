use std::{
    hash::Hash,
    iter::{self, Sum},
};

pub mod range_set;
pub mod rectangle;

use hashbrown::HashSet;
use num_traits::Num;
use range_set::RangeSet;

pub trait Rangelike: Sized {
    type Inner: Rangelike + Hash + Eq;
    type Unit: Num + Sum;

    fn is_empty(&self) -> bool;
    fn count(&self) -> Self::Unit;

    fn intersects(&self, other: &Self) -> bool;
    fn contains(&self, other: &Self) -> bool;

    fn intersection(&self, other: &Self) -> Option<Self::Inner>;
    fn difference(&self, other: &Self) -> RangeSet<Self::Inner>;
    fn union(&self, other: &Self) -> RangeSet<Self::Inner>;
    fn symmetric_difference(&self, other: &Self) -> RangeSet<Self::Inner>;
}

#[derive(PartialEq, Hash)]
pub struct Point<T: Num> {
    x: T,
    y: T,
}

impl<T: Num> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
