use hashbrown::HashSet;
use num_traits::Num;

use crate::Rangelike;
use std::hash::Hash;

pub struct RangeSet<T: Rangelike>(HashSet<T>);

impl<T: Rangelike> RangeSet<T> {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}

impl<T: Rangelike> Default for RangeSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Rangelike> IntoIterator for &'a RangeSet<T> {
    type IntoIter = hashbrown::hash_set::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> hashbrown::hash_set::Iter<'a, T> {
        self.0.iter()
    }
}

impl<T: Rangelike + Eq + Hash> FromIterator<T> for RangeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl<T: Rangelike + Eq + Hash> Rangelike for RangeSet<T> {
    type Inner = T;
    type Unit = T::Unit;

    fn is_empty(&self) -> bool {
        self.into_iter().all(|a| a.is_empty())
    }

    fn count(&self) -> Self::Unit {
        self.into_iter()
            .map(|a| a.count())
            .sum()
    }

    fn intersects(&self, other: &Self) -> bool {}

    fn contains(&self, other: &Self) -> bool {
        todo!()
    }

    fn intersection(&self, other: &Self) -> Option<Self::Inner> {
        todo!()
    }

    fn difference(&self, other: &Self) -> RangeSet<Self::Inner> {
        todo!()
    }

    fn union(&self, other: &Self) -> RangeSet<Self::Inner> {
        todo!()
    }

    fn symmetric_difference(&self, other: &Self) -> RangeSet<Self::Inner> {
        todo!()
    }
}
