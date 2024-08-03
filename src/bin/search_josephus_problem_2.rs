#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::iter::{FromIterator, FusedIterator};
use std::ops::{AddAssign, Bound, Index, RangeBounds, SubAssign};

// Fenwick Tree - https://github.com/brurucy/ftree
#[derive(Debug, Clone, PartialEq)]
pub struct FenwickTree<T> {
    inner: Vec<T>,
}

impl<T> FromIterator<T> for FenwickTree<T>
where
    T: Copy + AddAssign,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut inner: Vec<T> = iter.into_iter().collect();
        let n = inner.len();

        for i in 0..n {
            let parent = i | (i + 1);
            if parent < n {
                let child = inner[i];
                inner[parent] += child;
            }
        }

        FenwickTree { inner }
    }
}

// compile error on CSES version of Rust
// impl<const N: usize> From<[usize; N]> for FenwickTree<usize> {
//     fn from(value: [usize; N]) -> Self {
//         value.into_iter().collect::<FenwickTree<_>>()
//     }
// }

impl<T> FenwickTree<T> {
    pub fn prefix_sum(&self, index: usize, mut sum: T) -> T
    where
        T: Copy + AddAssign,
    {
        assert!(index < self.inner.len() + 1);

        let mut current_idx = index;

        while current_idx > 0 {
            sum += self.inner[current_idx - 1];
            current_idx &= current_idx - 1;
        }

        sum
    }

    pub fn add_at(&mut self, index: usize, diff: T)
    where
        T: Copy + AddAssign,
    {
        let mut current_idx = index;

        while let Some(value) = self.inner.get_mut(current_idx) {
            *value += diff;
            current_idx |= current_idx + 1;
        }
    }

    pub fn sub_at(&mut self, index: usize, diff: T)
    where
        T: Copy + SubAssign,
    {
        let mut current_idx = index;

        while let Some(value) = self.inner.get_mut(current_idx) {
            *value -= diff;
            current_idx |= current_idx + 1;
        }
    }

    pub fn index_of(&self, mut prefix_sum: T) -> usize
    where
        T: Copy + Ord + SubAssign,
    {
        let mut index = 0;
        let mut probe: usize = if self.inner.is_empty() {
            0
        } else {
            2 << (usize::BITS - 1 - self.inner.len().leading_zeros())
        };

        while probe > 0 {
            let lsb = probe & probe.wrapping_neg();
            let half_lsb = lsb / 2;
            let other_half_lsb = lsb - half_lsb;

            if let Some(value) = self.inner.get(probe - 1) {
                if *value < prefix_sum {
                    index = probe;
                    prefix_sum -= *value;

                    probe += half_lsb;

                    if half_lsb > 0 {
                        continue;
                    }
                }
            }

            if lsb % 2 > 0 {
                break;
            }

            probe -= other_half_lsb;
        }

        index
    }
}

// Indexed Set - https://github.com/brurucy/indexset

const DEFAULT_INNER_SIZE: usize = 1024;
const CUTOFF_RATIO: usize = 2;
const DEFAULT_CUTOFF: usize = DEFAULT_INNER_SIZE / CUTOFF_RATIO;

#[derive(Clone, Debug, PartialEq)]
struct Node<T>
where
    T: PartialOrd + Clone,
{
    pub inner: Vec<T>,
    pub max: Option<T>,
    pub iterations: usize,
}

impl<T: PartialOrd + Clone> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.max.partial_cmp(&other.max)
    }
}

impl<T: PartialOrd + Clone> Default for Node<T> {
    fn default() -> Self {
        Self {
            inner: Vec::with_capacity(DEFAULT_INNER_SIZE),
            max: None,
            iterations: 10,
        }
    }
}

fn search<T: PartialOrd>(haystack: &[T], needle: &T, iterations: usize) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = haystack.len();
    for _ in 0..iterations {
        if left >= right {
            break;
        }

        let mid = left + (right - left) / 2;

        let mid_value = unsafe { haystack.get_unchecked(mid) };

        if mid_value < needle {
            left = mid + 1;
        } else if mid_value > needle {
            right = mid;
        } else {
            return Ok(mid);
        }
    }

    Err(left)
}

impl<T: Ord + Clone> Node<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            iterations: capacity.ilog2() as usize,
            ..Default::default()
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
    pub fn split_off(&mut self, cutoff: usize) -> Self {
        let latter_inner = self.inner.split_off(cutoff);

        self.max = self.inner.last().cloned();

        let latter_inner_max = latter_inner.last().cloned();
        Self {
            inner: latter_inner,
            max: latter_inner_max,
            iterations: self.iterations,
        }
    }
    pub fn halve(&mut self) -> Self {
        self.split_off(DEFAULT_CUTOFF)
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn insert(&mut self, value: T) -> bool {
        match search(&self.inner, &value, self.iterations) {
            Ok(_) => return false,
            Err(idx) => {
                let some_value = Some(&value);
                if some_value > self.max.as_ref() {
                    self.max = some_value.cloned();
                }

                self.inner.insert(idx, value);
            }
        }

        true
    }
    pub fn delete(&mut self, index: usize) -> T {
        self.inner.remove(index)
    }
}

#[derive(Debug, Clone)]
pub struct IndexSet<T>
where
    T: Clone + Ord,
{
    inner: Vec<Node<T>>,
    index: FenwickTree<usize>,
    node_capacity: usize,
    len: usize,
}

impl<T: Clone + Ord> IndexSet<T> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_maximum_node_size(maximum_node_size: usize) -> Self {
        let mut new: Self = IndexSet::default();
        new.inner = vec![Node::new(maximum_node_size)];

        new
    }

    pub fn clear(&mut self) {
        self.inner = vec![Node::new(self.node_capacity)];
        self.index = FenwickTree::from_iter(vec![0]);
        self.len = 0;
    }

    fn locate_node<Q>(&self, value: &Q) -> usize
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut node_idx = self.inner.partition_point(|node| {
            if let Some(max) = node.max.as_ref() {
                return max.borrow() < value;
            };

            false
        });

        if self.inner.get(node_idx).is_none() {
            node_idx -= 1;
        }

        node_idx
    }

    fn locate_value<Q>(&self, value: &Q) -> (usize, usize)
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let node_idx = self.locate_node(value);
        let position_within_node = self.inner[node_idx]
            .inner
            .partition_point(|item| item.borrow() < value);

        (node_idx, position_within_node)
    }

    fn locate_ith(&self, idx: usize) -> (usize, usize) {
        let mut node_index = self.index.index_of(idx);
        let mut offset = 0;

        if node_index != 0 {
            offset = self.index.prefix_sum(node_index, 0);
        }

        let mut position_within_node = idx - offset;
        if let Some(node) = self.inner.get(node_index) {
            if position_within_node == node.len() {
                node_index += 1;
                position_within_node = 0;
            }
        }

        (node_index, position_within_node)
    }

    pub fn get_index(&self, idx: usize) -> Option<&T> {
        let (node_idx, position_within_node) = self.locate_ith(idx);
        if let Some(candidate_node) = self.inner.get(node_idx) {
            return candidate_node.get(position_within_node);
        }

        None
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        let (node_idx, position_within_node) = self.locate_value(value);
        if let Some(candidate_node) = self.inner.get(node_idx) {
            return candidate_node.get(position_within_node);
        }

        None
    }

    pub fn lower_bound<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let (node_idx, position_within_node) = self.locate_value(value);
        if let Some(candidate_node) = self.inner.get(node_idx) {
            return candidate_node.get(position_within_node);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn insert(&mut self, value: T) -> bool {
        let node_idx = self.locate_node(&value);
        if self.inner[node_idx].len() == DEFAULT_INNER_SIZE {
            let new_node = self.inner[node_idx].halve();
            // Get the minimum
            let new_node_min = new_node.inner[0].clone();
            // Insert the new node
            self.inner.insert(node_idx + 1, new_node);
            let insert_node_idx = if value < new_node_min {
                node_idx
            } else {
                node_idx + 1
            };
            if self.inner[insert_node_idx].insert(value) {
                // Reconstruct the index after the new node insert.
                self.index = self.inner.iter().map(Node::len).collect::<FenwickTree<_>>();
                self.len += 1;
                true
            } else {
                false
            }
        } else if self.inner[node_idx].insert(value) {
            self.index.add_at(node_idx, 1);
            self.len += 1;
            true
        } else {
            false
        }
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        let replaced_element = self.take(&value);
        self.insert(value);

        replaced_element
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let (node_idx, position_within_node) = self.locate_value(value);
        if let Some(candidate_node) = self.inner.get(node_idx) {
            if let Some(candidate_value) = candidate_node.get(position_within_node) {
                return value == candidate_value.borrow();
            }
        }

        false
    }

    fn delete_at(&mut self, node_idx: usize, position_within_node: usize) -> T {
        let removal = self.inner[node_idx].delete(position_within_node);

        let mut decrease_length = false;
        if self.inner[node_idx].len() == 0 {
            if self.inner.len() > 1 {
                self.inner.remove(node_idx);
                self.len -= 1;
                self.index = self.inner.iter().map(Node::len).collect::<FenwickTree<_>>();
            } else {
                decrease_length = true;
            }
        } else {
            decrease_length = true;
        }

        if decrease_length {
            self.index.sub_at(node_idx, 1);
            self.len -= 1;
        }

        removal
    }
    fn delete<Q>(&mut self, value: &Q) -> (Option<T>, bool)
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut removed = false;
        let mut removal = None;
        let (node_idx, position_within_node) = self.locate_value(value);
        if let Some(candidate_node) = self.inner.get(node_idx) {
            if let Some(candidate_value) = candidate_node.get(position_within_node) {
                if value == candidate_value.borrow() {
                    removal = Some(self.delete_at(node_idx, position_within_node));
                    removed = true;
                }
            }
        }

        (removal, removed)
    }

    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.delete(value).1
    }

    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.delete(value).0
    }

    pub fn first(&self) -> Option<&T> {
        if let Some(candidate_node) = self.inner.first() {
            return candidate_node.get(0);
        }

        None
    }

    pub fn last(&self) -> Option<&T> {
        if let Some(candidate_node) = self.inner.last() {
            if candidate_node.len() > 0 {
                return candidate_node.get(candidate_node.len() - 1);
            }
        }

        None
    }

    pub fn pop_first(&mut self) -> Option<T> {
        let (first_node_idx, first_position_within_node) = (0, 0);
        if let Some(candidate_node) = self.inner.get(first_node_idx) {
            if candidate_node.get(first_position_within_node).is_some() {
                return Some(self.delete_at(first_node_idx, first_position_within_node));
            }
        }

        None
    }

    pub fn pop_index(&mut self, idx: usize) -> T {
        let (node_idx, position_within_node) = self.locate_ith(idx);

        self.delete_at(node_idx, position_within_node)
    }

    pub fn pop_last(&mut self) -> Option<T> {
        let last_node_idx = self.inner.len() - 1;
        let mut last_position_within_node = self.inner[last_node_idx].inner.len();
        last_position_within_node = last_position_within_node.saturating_sub(1);

        if let Some(candidate_node) = self.inner.get(last_node_idx) {
            if candidate_node.get(last_position_within_node).is_some() {
                return Some(self.delete_at(last_node_idx, last_position_within_node));
            }
        }

        None
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.difference(other).next().is_some() {
            return false;
        }

        true
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        if other.difference(self).next().is_some() {
            return false;
        }

        true
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.intersection(other).next().is_some() {
            return false;
        }

        true
    }

    pub fn iter(&self) -> Iter<T> {
        return Iter::new(self);
    }

    pub fn union<'a>(&'a self, other: &'a Self) -> Union<T> {
        return Union {
            merge_iter: MergeIter {
                start: true,
                left_iter: self.iter(),
                current_left: None,
                right_iter: other.iter(),
                current_right: None,
            },
        };
    }

    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<T> {
        return Difference {
            merge_iter: MergeIter {
                start: true,
                left_iter: self.iter(),
                current_left: None,
                right_iter: other.iter(),
                current_right: None,
            },
        };
    }

    pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<T> {
        return SymmetricDifference {
            merge_iter: MergeIter {
                start: true,
                left_iter: self.iter(),
                current_left: None,
                right_iter: other.iter(),
                current_right: None,
            },
        };
    }

    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<T> {
        return Intersection {
            merge_iter: MergeIter {
                start: true,
                left_iter: self.iter(),
                current_left: None,
                right_iter: other.iter(),
                current_right: None,
            },
        };
    }

    pub fn retain<F, Q>(&mut self, mut f: F)
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
        F: FnMut(&Q) -> bool,
    {
        let mut positions_to_delete = vec![];
        for (node_idx, node) in self.inner.iter().enumerate() {
            for (position_within_node, item) in node.inner.iter().enumerate() {
                if !f(item.borrow()) {
                    positions_to_delete.push((node_idx, position_within_node));
                }
            }
        }
        positions_to_delete.reverse();

        for (node_idx, position_within_node) in positions_to_delete {
            self.delete_at(node_idx, position_within_node);
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        while let Some(value) = other.pop_first() {
            self.replace(value);
        }
    }
    fn resolve_range<R>(&self, range: R) -> ((usize, usize, usize), (usize, usize, usize))
    where
        R: RangeBounds<usize>,
    {
        let mut global_front_idx: usize = 0;
        let mut global_back_idx: usize =
            self.index.prefix_sum(self.inner.len(), 0).saturating_sub(1);

        // Solving global indexes
        let start = range.start_bound();
        match start {
            Bound::Included(bound) => {
                global_front_idx = *bound;
            }
            Bound::Excluded(bound) => {
                global_front_idx = *bound + 1;
            }
            Bound::Unbounded => (),
        }

        let end = range.end_bound();
        match end {
            Bound::Included(bound) => {
                global_back_idx = *bound;
            }
            Bound::Excluded(bound) => {
                global_back_idx = *bound - 1;
            }
            Bound::Unbounded => (),
        }
        // Figuring out nodes
        let (front_node_idx, front_start_idx) = self.locate_ith(global_front_idx);
        let (back_node_idx, back_start_idx) = self.locate_ith(global_back_idx);

        (
            (global_front_idx, front_node_idx, front_start_idx),
            (global_back_idx, back_node_idx, back_start_idx),
        )
    }

    pub fn range<R, Q>(&self, range: R) -> Range<'_, T>
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
        R: RangeBounds<Q>,
    {
        let start_idx = match range.start_bound() {
            Bound::Included(bound) => self.rank(bound),
            Bound::Excluded(bound) => self.rank(bound) + 1,
            Bound::Unbounded => 0,
        };
        let end_idx = match range.end_bound() {
            Bound::Included(bound) => self.rank(bound),
            Bound::Excluded(bound) => self.rank(bound).saturating_sub(1),
            Bound::Unbounded => self.len().saturating_sub(1),
        };

        self.range_idx(start_idx..(end_idx + 1))
    }

    pub fn rank<Q>(&self, value: &Q) -> usize
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        let (node_idx, position_within_node) = self.locate_value(value);

        let offset = self.index.prefix_sum(node_idx, 0);

        offset + position_within_node
    }

    fn range_idx<R>(&self, range: R) -> Range<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let (
            (global_front_idx, front_node_idx, front_start_idx),
            (global_back_idx, back_node_idx, back_start_idx),
        ) = self.resolve_range(range);

        let front_iter = if front_node_idx < self.inner.len() {
            Some(self.inner[front_node_idx].inner[front_start_idx..].iter())
        } else {
            None
        };

        let back_iter = if back_node_idx < self.inner.len() {
            Some(self.inner[back_node_idx].inner[..(back_start_idx + 1)].iter())
        } else {
            None
        };

        Range {
            spine_iter: Iter {
                btree: self,
                current_front_node_idx: front_node_idx,
                current_front_idx: global_front_idx,
                current_back_node_idx: back_node_idx,
                current_back_idx: global_back_idx + 1,
                current_front_iterator: front_iter,
                current_back_iterator: back_iter,
            },
        }
    }
}

impl<T> FromIterator<T> for IndexSet<T>
where
    T: Ord + Clone,
{
    fn from_iter<K: IntoIterator<Item = T>>(iter: K) -> Self {
        let mut btree = IndexSet::new();
        iter.into_iter().for_each(|item| {
            btree.insert(item);
        });

        btree
    }
}

impl<T, const N: usize> From<[T; N]> for IndexSet<T>
where
    T: Ord + Clone,
{
    fn from(value: [T; N]) -> Self {
        let mut btree: IndexSet<T> = IndexSet::default();

        for item in value {
            btree.insert(item);
        }

        btree
    }
}

impl<T> Default for IndexSet<T>
where
    T: Clone + Ord,
{
    fn default() -> Self {
        let node_capacity = DEFAULT_INNER_SIZE;

        Self {
            inner: vec![Node::new(node_capacity)],
            index: FenwickTree::from_iter(vec![0]),
            node_capacity,
            len: 0,
        }
    }
}

pub struct Iter<'a, T>
where
    T: Clone + Ord,
{
    btree: &'a IndexSet<T>,
    current_front_node_idx: usize,
    current_front_idx: usize,
    current_back_node_idx: usize,
    current_back_idx: usize,
    current_front_iterator: Option<std::slice::Iter<'a, T>>,
    current_back_iterator: Option<std::slice::Iter<'a, T>>,
}

impl<'a, T> Iter<'a, T>
where
    T: Clone + Ord,
{
    pub fn new(btree: &'a IndexSet<T>) -> Self {
        return Self {
            btree,
            current_front_node_idx: 0,
            current_front_idx: 0,
            current_back_node_idx: btree.inner.len() - 1,
            current_back_idx: btree.len(),
            current_front_iterator: Some(btree.inner[0].inner.iter()),
            current_back_iterator: Some(btree.inner[btree.inner.len() - 1].inner.iter()),
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Clone + Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_front_idx == self.current_back_idx {
            return None;
        }
        return if let Some(value) = self
            .current_front_iterator
            .as_mut()
            .and_then(std::iter::Iterator::next)
        {
            self.current_front_idx += 1;
            Some(value)
        } else {
            self.current_front_node_idx += 1;
            if self.current_front_node_idx >= self.btree.inner.len() {
                return None;
            }
            self.current_front_iterator =
                Some(self.btree.inner[self.current_front_node_idx].inner.iter());

            self.next()
        };
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T>
where
    T: Clone + Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_front_idx == self.current_back_idx {
            return None;
        }
        return if let Some(value) = self
            .current_back_iterator
            .as_mut()
            .and_then(std::iter::DoubleEndedIterator::next_back)
        {
            self.current_back_idx -= 1;
            Some(value)
        } else {
            if self.current_back_node_idx == 0 {
                return None;
            };
            self.current_back_node_idx -= 1;
            self.current_back_iterator =
                Some(self.btree.inner[self.current_back_node_idx].inner.iter());

            self.next_back()
        };
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> where T: Clone + Ord {}

impl<'a, T> IntoIterator for &'a IndexSet<T>
where
    T: Clone + Ord,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct IntoIter<T>
where
    T: Clone + Ord,
{
    btree: IndexSet<T>,
}

impl<T> Iterator for IntoIter<T>
where
    T: Clone + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.btree.pop_first()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
where
    T: Clone + Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.btree.pop_last()
    }
}

impl<T> FusedIterator for IntoIter<T> where T: Clone + Ord {}

impl<T> IntoIterator for IndexSet<T>
where
    T: Clone + Ord,
{
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        // This will never panic, since there always is at least one node in the btree
        IntoIter { btree: self }
    }
}

struct MergeIter<'a, T>
where
    T: Clone + Ord,
{
    start: bool,
    left_iter: Iter<'a, T>,
    current_left: Option<&'a T>,
    right_iter: Iter<'a, T>,
    current_right: Option<&'a T>,
}

impl<'a, T> Iterator for MergeIter<'a, T>
where
    T: Clone + Ord,
{
    type Item = (Option<&'a T>, Option<&'a T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.current_left = self.left_iter.next();
            self.current_right = self.right_iter.next();
            self.start = false;
        } else if let Some(left) = self.current_left {
            if let Some(right) = self.current_right {
                match left.cmp(right) {
                    Ordering::Less => {
                        self.current_left = self.left_iter.next();
                    }
                    Ordering::Equal => {
                        self.current_left = self.left_iter.next();
                        self.current_right = self.right_iter.next();
                    }
                    Ordering::Greater => {
                        self.current_right = self.right_iter.next();
                    }
                }
            } else {
                self.current_left = self.left_iter.next();
            }
        } else if self.current_right.is_some() {
            self.current_right = self.right_iter.next();
        } else {
            return None;
        }

        Some((self.current_left, self.current_right))
    }
}

pub struct Union<'a, T>
where
    T: Clone + Ord,
{
    merge_iter: MergeIter<'a, T>,
}

impl<'a, T> Iterator for Union<'a, T>
where
    T: Clone + Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((current_left, current_right)) = self.merge_iter.next() {
            return match (current_left, current_right) {
                (Some(left), Some(right)) => {
                    if right < left {
                        Some(right)
                    } else {
                        Some(left)
                    }
                }
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (None, None) => None,
            };
        }

        None
    }
}

impl<'a, T> FusedIterator for Union<'a, T> where T: Clone + Ord {}

pub struct Difference<'a, T>
where
    T: Clone + Ord,
{
    merge_iter: MergeIter<'a, T>,
}

impl<'a, T> Iterator for Difference<'a, T>
where
    T: Clone + Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return if let Some((current_left, current_right)) = self.merge_iter.next() {
                match (current_left, current_right) {
                    (Some(left), Some(right)) => {
                        if left < right {
                            Some(left)
                        } else {
                            continue;
                        }
                    }
                    (Some(left), None) => Some(left),
                    (None, _) => None,
                }
            } else {
                None
            };
        }
    }
}

impl<'a, T> FusedIterator for Difference<'a, T> where T: Clone + Ord {}

pub struct SymmetricDifference<'a, T>
where
    T: Clone + Ord,
{
    merge_iter: MergeIter<'a, T>,
}

impl<'a, T> Iterator for SymmetricDifference<'a, T>
where
    T: Clone + Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            return if let Some((current_left, current_right)) = self.merge_iter.next() {
                match (current_left, current_right) {
                    (Some(left), Some(right)) =>
                    {
                        #[allow(clippy::comparison_chain)]
                        if left < right {
                            Some(left)
                        } else if right < left {
                            Some(right)
                        } else {
                            continue;
                        }
                    }
                    (Some(left), None) => Some(left),
                    (None, Some(right)) => Some(right),
                    (None, _) => None,
                }
            } else {
                None
            };
        }
    }
}

impl<'a, T> FusedIterator for SymmetricDifference<'a, T> where T: Clone + Ord {}

pub struct Intersection<'a, T>
where
    T: Clone + Ord,
{
    merge_iter: MergeIter<'a, T>,
}

impl<'a, T> Iterator for Intersection<'a, T>
where
    T: Clone + Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((current_left, current_right)) = self.merge_iter.next() {
                match (current_left, current_right) {
                    (Some(left), Some(right)) => {
                        if left == right {
                            return Some(left);
                        }
                        continue;
                    }
                    (None, _) | (_, None) => return None,
                }
            }
            return None;
        }
    }
}

impl<'a, T> FusedIterator for Intersection<'a, T> where T: Clone + Ord {}

pub struct Range<'a, T>
where
    T: Clone + Ord,
{
    spine_iter: Iter<'a, T>,
}

impl<'a, T> Iterator for Range<'a, T>
where
    T: Clone + Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.spine_iter.next()
    }
}

impl<'a, T> DoubleEndedIterator for Range<'a, T>
where
    T: Clone + Ord,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.spine_iter.next_back()
    }
}

impl<'a, T> FusedIterator for Range<'a, T> where T: Clone + Ord {}

impl<T> Index<usize> for IndexSet<T>
where
    T: Ord + Clone,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index).unwrap()
    }
}

// I/O boilerplate //

use std::io::Read;

/// https://github.com/Kogia-sima/itoap
#[allow(clippy::pedantic)]
pub mod itoap {
    mod common {
        use core::ops::{Div, Mul, Sub};
        use core::ptr;

        const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

        #[inline]
        pub fn divmod<T: Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T>>(
            x: T,
            y: T,
        ) -> (T, T) {
            let quot = x / y;
            let rem = x - quot * y;
            (quot, rem)
        }

        #[inline]
        pub unsafe fn lookup<T: Into<u64>>(idx: T) -> *const u8 {
            DEC_DIGITS_LUT.as_ptr().add((idx.into() as usize) << 1)
        }

        #[inline]
        pub unsafe fn write4(n: u32, buf: *mut u8) -> usize {
            debug_assert!(n < 10000);

            if n < 100 {
                if n < 10 {
                    *buf = n as u8 + 0x30;
                    1
                } else {
                    ptr::copy_nonoverlapping(lookup(n), buf, 2);
                    2
                }
            } else {
                let (n1, n2) = divmod(n, 100);
                if n < 1000 {
                    *buf = n1 as u8 + 0x30;
                    ptr::copy_nonoverlapping(lookup(n2), buf.add(1), 2);
                    3
                } else {
                    ptr::copy_nonoverlapping(lookup(n1), buf.add(0), 2);
                    ptr::copy_nonoverlapping(lookup(n2), buf.add(2), 2);
                    4
                }
            }
        }

        #[inline]
        pub unsafe fn write4_pad(n: u32, buf: *mut u8) {
            debug_assert!(n < 10000);
            let (n1, n2) = divmod(n, 100);

            ptr::copy_nonoverlapping(lookup(n1), buf, 2);
            ptr::copy_nonoverlapping(lookup(n2), buf.add(2), 2);
        }

        #[inline]
        pub unsafe fn write8(n: u32, buf: *mut u8) -> usize {
            debug_assert!(n < 100_000_000);

            if n < 10000 {
                write4(n, buf)
            } else {
                let (n1, n2) = divmod(n, 10000);

                let l = if n1 < 100 {
                    if n1 < 10 {
                        *buf = n1 as u8 + 0x30;
                        5
                    } else {
                        ptr::copy_nonoverlapping(lookup(n1), buf, 2);
                        6
                    }
                } else {
                    let (n11, n12) = divmod(n1, 100);
                    if n1 < 1000 {
                        *buf = n11 as u8 + 0x30;
                        ptr::copy_nonoverlapping(lookup(n12), buf.add(1), 2);
                        7
                    } else {
                        ptr::copy_nonoverlapping(lookup(n11), buf.add(0), 2);
                        ptr::copy_nonoverlapping(lookup(n12), buf.add(2), 2);
                        8
                    }
                };

                let (n21, n22) = divmod(n2, 100);
                ptr::copy_nonoverlapping(lookup(n21), buf.add(l - 4), 2);
                ptr::copy_nonoverlapping(lookup(n22), buf.add(l - 2), 2);
                l
            }
        }

        #[inline]
        pub unsafe fn write8_pad(n: u32, buf: *mut u8) {
            debug_assert!(n < 100_000_000);

            let (n1, n2) = divmod(n, 10000);
            let (n11, n12) = divmod(n1, 100);
            let (n21, n22) = divmod(n2, 100);

            ptr::copy_nonoverlapping(lookup(n11), buf, 2);
            ptr::copy_nonoverlapping(lookup(n12), buf.add(2), 2);
            ptr::copy_nonoverlapping(lookup(n21), buf.add(4), 2);
            ptr::copy_nonoverlapping(lookup(n22), buf.add(6), 2);
        }

        pub unsafe fn write_u8(n: u8, buf: *mut u8) -> usize {
            if n < 10 {
                *buf = n + 0x30;
                1
            } else if n < 100 {
                ptr::copy_nonoverlapping(lookup(n), buf, 2);
                2
            } else {
                let (n1, n2) = divmod(n, 100);
                *buf = n1 + 0x30;
                ptr::copy_nonoverlapping(lookup(n2), buf.add(1), 2);
                3
            }
        }

        pub unsafe fn write_u16(n: u16, buf: *mut u8) -> usize {
            if n < 100 {
                if n < 10 {
                    *buf = n as u8 + 0x30;
                    1
                } else {
                    ptr::copy_nonoverlapping(lookup(n), buf, 2);
                    2
                }
            } else if n < 10000 {
                let (a1, a2) = divmod(n, 100);

                if n < 1000 {
                    *buf = a1 as u8 + 0x30;
                    ptr::copy_nonoverlapping(lookup(a2), buf.add(1), 2);
                    3
                } else {
                    ptr::copy_nonoverlapping(lookup(a1), buf, 2);
                    ptr::copy_nonoverlapping(lookup(a2), buf.add(2), 2);
                    4
                }
            } else {
                let (a1, a2) = divmod(n, 10000);
                let (b1, b2) = divmod(a2, 100);

                *buf = a1 as u8 + 0x30;
                ptr::copy_nonoverlapping(lookup(b1), buf.add(1), 2);
                ptr::copy_nonoverlapping(lookup(b2), buf.add(3), 2);
                5
            }
        }

        #[inline]
        fn u128_mulhi(x: u128, y: u128) -> u128 {
            let x_lo = x as u64;
            let x_hi = (x >> 64) as u64;
            let y_lo = y as u64;
            let y_hi = (y >> 64) as u64;

            let carry = (x_lo as u128 * y_lo as u128) >> 64;
            let m = x_lo as u128 * y_hi as u128 + carry;
            let high1 = m >> 64;

            let m_lo = m as u64;
            let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

            x_hi as u128 * y_hi as u128 + high1 + high2
        }

        unsafe fn write_u128_big(mut n: u128, mut buf: *mut u8) -> usize {
            const DIV_FACTOR: u128 = 76624777043294442917917351357515459181;
            const DIV_SHIFT: u32 = 51;
            const POW_10_8: u64 = 100000000;
            const POW_10_16: u64 = 10000000000000000;

            debug_assert!(n > u64::MAX as u128);

            let mut result = [0u32; 5];

            {
                let quot = u128_mulhi(n, DIV_FACTOR) >> DIV_SHIFT;
                let rem = (n - quot * POW_10_16 as u128) as u64;
                debug_assert_eq!(quot, n / POW_10_16 as u128);
                debug_assert_eq!(rem as u128, n % POW_10_16 as u128);

                n = quot;

                result[1] = (rem / POW_10_8) as u32;
                result[0] = (rem % POW_10_8) as u32;

                debug_assert_ne!(n, 0);
                debug_assert!(n <= u128::MAX / POW_10_16 as u128);
            }

            let result_len = if n >= POW_10_16 as u128 {
                let quot = (n >> 16) as u64 / (POW_10_16 >> 16);
                let rem = (n - POW_10_16 as u128 * quot as u128) as u64;
                debug_assert_eq!(quot as u128, n / POW_10_16 as u128);
                debug_assert_eq!(rem as u128, n % POW_10_16 as u128);
                debug_assert!(quot <= 3402823);

                result[3] = (rem / POW_10_8) as u32;
                result[2] = (rem % POW_10_8) as u32;
                result[4] = quot as u32;
                4
            } else if (n as u64) >= POW_10_8 {
                result[3] = ((n as u64) / POW_10_8) as u32;
                result[2] = ((n as u64) % POW_10_8) as u32;
                3
            } else {
                result[2] = n as u32;
                2
            };

            let l = write8(*result.get_unchecked(result_len), buf);
            buf = buf.add(l);

            for i in (0..result_len).rev() {
                write8_pad(*result.get_unchecked(i), buf);
                buf = buf.add(8);
            }

            l + result_len * 8
        }

        #[inline]
        pub unsafe fn write_u128(n: u128, buf: *mut u8) -> usize {
            if n <= u64::MAX as u128 {
                super::write_u64(n as u64, buf)
            } else {
                write_u128_big(n, buf)
            }
        }
    }
    use common::*;

    #[cfg(not(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    )))]
    mod fallback {
        use core::ptr;

        use super::common::{divmod, lookup, write4, write4_pad, write8_pad};

        pub unsafe fn write_u32(n: u32, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n, 10000);

                let l = write4(n1, buf);
                write4_pad(n2, buf.add(l));
                l + 4
            } else {
                let (n1, n2) = divmod(n, 100_000_000);

                let l = if n1 >= 10 {
                    ptr::copy_nonoverlapping(lookup(n1), buf, 2);
                    2
                } else {
                    *buf = n1 as u8 + 0x30;
                    1
                };

                write8_pad(n2, buf.add(l));
                l + 8
            }
        }

        pub unsafe fn write_u64(n: u64, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n as u32, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n, 10000);

                let l = write4(n1 as u32, buf);
                write4_pad(n2 as u32, buf.add(l));
                l + 4
            } else if n < 10_000_000_000_000_000 {
                let (n1, n2) = divmod(n, 100_000_000);
                let (n1, n2) = (n1 as u32, n2 as u32);

                let l = if n1 < 10000 {
                    write4(n1, buf)
                } else {
                    let (n11, n12) = divmod(n1, 10000);
                    let l = write4(n11, buf);
                    write4_pad(n12, buf.add(l));
                    l + 4
                };

                write8_pad(n2, buf.add(l));
                l + 8
            } else {
                let (n1, n2) = divmod(n, 10_000_000_000_000_000);
                let (n21, n22) = divmod(n2, 100_000_000);

                let l = write4(n1 as u32, buf);
                write8_pad(n21 as u32, buf.add(l));
                write8_pad(n22 as u32, buf.add(l + 8));
                l + 16
            }
        }
    }

    #[cfg(not(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    )))]
    use fallback::{write_u32, write_u64};

    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    ))]
    mod sse2 {
        #![allow(non_upper_case_globals)]

        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        use super::common::{divmod, lookup, write4, write4_pad};
        use core::ptr;

        #[repr(align(16))]
        struct Aligned<T>(T);

        impl<T> std::ops::Deref for Aligned<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &T {
                &self.0
            }
        }

        const kDiv10000: u32 = 0xd1b71759;
        const kDivPowersVector: Aligned<[u16; 8]> =
            Aligned([8389, 5243, 13108, 32768, 8389, 5243, 13108, 32768]);
        const kShiftPowersVector: Aligned<[u16; 8]> = Aligned([
            1 << (16 - (23 + 2 - 16)),
            1 << (16 - (19 + 2 - 16)),
            1 << (16 - 1 - 2),
            1 << (15),
            1 << (16 - (23 + 2 - 16)),
            1 << (16 - (19 + 2 - 16)),
            1 << (16 - 1 - 2),
            1 << (15),
        ]);

        #[inline]
        unsafe fn convert_8digits_sse2(value: u32) -> __m128i {
            debug_assert!(value <= 99999999);

            let abcdefgh = _mm_cvtsi32_si128(value as i32);
            let abcd = _mm_srli_epi64(
                _mm_mul_epu32(abcdefgh, _mm_set1_epi32(kDiv10000 as i32)),
                45,
            );
            let efgh = _mm_sub_epi32(abcdefgh, _mm_mul_epu32(abcd, _mm_set1_epi32(10000)));

            let v1 = _mm_unpacklo_epi16(abcd, efgh);

            let v1a = _mm_slli_epi64(v1, 2);

            let v2a = _mm_unpacklo_epi16(v1a, v1a);
            let v2 = _mm_unpacklo_epi32(v2a, v2a);

            let v3 = _mm_mulhi_epu16(
                v2,
                _mm_load_si128(kDivPowersVector.as_ptr() as *const __m128i),
            );
            let v4 = _mm_mulhi_epu16(
                v3,
                _mm_load_si128(kShiftPowersVector.as_ptr() as *const __m128i),
            );

            let v5 = _mm_mullo_epi16(v4, _mm_set1_epi16(10));

            let v6 = _mm_slli_epi64(v5, 16);

            _mm_sub_epi16(v4, v6)
        }

        pub unsafe fn write_u32(n: u32, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n, 10000);

                let l = write4(n1, buf);
                write4_pad(n2, buf.add(l));
                l + 4
            } else {
                let (n1, n2) = divmod(n, 100_000_000);

                let l = if n1 >= 10 {
                    ptr::copy_nonoverlapping(lookup(n1), buf, 2);
                    2
                } else {
                    *buf = n1 as u8 + 0x30;
                    1
                };

                let b = convert_8digits_sse2(n2);
                let ba = _mm_add_epi8(
                    _mm_packus_epi16(_mm_setzero_si128(), b),
                    _mm_set1_epi8(b'0' as i8),
                );
                let result = _mm_srli_si128(ba, 8);
                _mm_storel_epi64(buf.add(l) as *mut __m128i, result);

                l + 8
            }
        }

        pub unsafe fn write_u64(n: u64, buf: *mut u8) -> usize {
            if n < 10000 {
                write4(n as u32, buf)
            } else if n < 100_000_000 {
                let (n1, n2) = divmod(n as u32, 10000);

                let l = write4(n1, buf);
                write4_pad(n2, buf.add(l));
                l + 4
            } else if n < 10_000_000_000_000_000 {
                let (n1, n2) = divmod(n, 100_000_000);
                let (n1, n2) = (n1 as u32, n2 as u32);

                let l = if n1 < 10000 {
                    write4(n1, buf)
                } else {
                    let (n11, n12) = divmod(n1, 10000);
                    let l = write4(n11, buf);
                    write4_pad(n12, buf.add(l));
                    l + 4
                };

                let b = convert_8digits_sse2(n2);
                let ba = _mm_add_epi8(
                    _mm_packus_epi16(_mm_setzero_si128(), b),
                    _mm_set1_epi8(b'0' as i8),
                );
                let result = _mm_srli_si128(ba, 8);
                _mm_storel_epi64(buf.add(l) as *mut __m128i, result);

                l + 8
            } else {
                let (n1, n2) = divmod(n, 10_000_000_000_000_000);
                let l = write4(n1 as u32, buf);

                let (n21, n22) = divmod(n2, 100_000_000);

                let a0 = convert_8digits_sse2(n21 as u32);
                let a1 = convert_8digits_sse2(n22 as u32);

                let va = _mm_add_epi8(_mm_packus_epi16(a0, a1), _mm_set1_epi8(b'0' as i8));
                _mm_storeu_si128(buf.add(l) as *mut __m128i, va);

                l + 16
            }
        }
    }

    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2",
        not(miri),
    ))]
    use sse2::{write_u32, write_u64};

    mod private {
        pub trait Sealed {}
    }

    pub trait Integer: private::Sealed {
        const MAX_LEN: usize;

        #[doc(hidden)]
        unsafe fn write_to(self, buf: *mut u8) -> usize;
    }

    macro_rules! impl_integer {
        ($unsigned:ty, $signed:ty, $conv:ty, $func:ident, $max_len:expr) => {
            impl private::Sealed for $unsigned {}
            impl private::Sealed for $signed {}

            impl Integer for $unsigned {
                const MAX_LEN: usize = $max_len;

                #[inline]
                unsafe fn write_to(self, buf: *mut u8) -> usize {
                    $func(self as $conv, buf)
                }
            }

            impl Integer for $signed {
                const MAX_LEN: usize = $max_len + 1;

                #[inline]
                unsafe fn write_to(self, mut buf: *mut u8) -> usize {
                    let mut n = self as $conv;
                    if self < 0 {
                        *buf = b'-';
                        buf = buf.add(1);
                        n = (!n).wrapping_add(1);
                    }

                    $func(n, buf) + (self < 0) as usize
                }
            }
        };
    }

    impl_integer!(u8, i8, u8, write_u8, 3);
    impl_integer!(u16, i16, u16, write_u16, 5);
    impl_integer!(u32, i32, u32, write_u32, 10);
    impl_integer!(u64, i64, u64, write_u64, 20);
    impl_integer!(u128, i128, u128, write_u128, 39);

    #[cfg(target_pointer_width = "16")]
    impl_integer!(usize, isize, u16, write_u16, 5);

    #[cfg(target_pointer_width = "32")]
    impl_integer!(usize, isize, u32, write_u32, 10);

    #[cfg(target_pointer_width = "64")]
    impl_integer!(usize, isize, u64, write_u64, 20);

    /// # Safety
    ///
    /// "buf" should have sufficient memory to write "value"
    #[inline]
    pub unsafe fn write_to_ptr<V: Integer>(buf: *mut u8, value: V) -> usize {
        value.write_to(buf)
    }
}

const BUF_SIZE: usize = 32_768;

pub struct CustomBufWriter<'a, W: std::io::Write> {
    writer: &'a mut W,
    buffer: [u8; BUF_SIZE],
    buffer_pointer: usize,
}

impl<'a, W: std::io::Write> CustomBufWriter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            buffer: [0; BUF_SIZE],
            buffer_pointer: 0,
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            self.writer
                .write_all(self.buffer.get_unchecked(..self.buffer_pointer))
                .unwrap_unchecked();
            self.buffer_pointer = 0;
        }
    }

    pub fn maybe_flush(&mut self, block_size: usize) {
        if self.buffer_pointer + block_size > BUF_SIZE {
            self.flush();
        }
    }

    pub fn add_int(&mut self, integer: impl itoap::Integer) {
        unsafe {
            self.buffer_pointer += itoap::write_to_ptr(
                self.buffer
                    .get_unchecked_mut(self.buffer_pointer..)
                    .as_mut_ptr(),
                integer,
            );
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        unsafe {
            self.buffer
                .as_mut_ptr()
                .add(self.buffer_pointer)
                .write(byte);
            self.buffer_pointer += 1;
        }
    }

    pub fn add_bytes(&mut self, buf: &[u8]) {
        unsafe {
            let len = buf.len();
            let ptr = self
                .buffer
                .get_unchecked_mut(self.buffer_pointer..)
                .as_mut_ptr();
            ptr.copy_from_nonoverlapping(buf.as_ptr(), len);
            self.buffer_pointer += len;
        }
    }
}

impl<'a, W: std::io::Write> Drop for CustomBufWriter<'a, W> {
    fn drop(&mut self) {
        self.flush();
    }
}

pub trait PosInt {
    fn to_posint(buf: &[u8]) -> Self;
}

macro_rules! impl_int {
    (for $($t:ty),+) => {
        $(impl PosInt for $t {
            #[allow(clippy::cast_lossless, clippy::cast_possible_wrap)]
            fn to_posint(buf: &[u8]) -> Self {
                unsafe {
                    buf.iter()
                        .map(|byte| (byte & 15) as $t)
                        .reduce(|acc, digit| acc * 10 + digit)
                        .unwrap_unchecked()
                }
            }
        })*
    }
}
impl_int!(for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

// problem //

/// Consider a game where there are n children (numbered 1,2,...,n) in a circle. During the game, repeatedly k children are skipped and one child is removed from the circle. In which order will the children be removed?
///
/// <b>Input</b>
///
/// The only input line has two integers n and k.
///
/// <b>Output</b>
///
/// Print n integers: the removal order.
///
/// <b>Constraints</b>
///
/// <ul>
/// <li>1 ≤ n ≤ 2 * 10<sup>5</sup></li>
/// <li>0 ≤ k ≤ 10<sup>9</sup></li>
/// </ul>
fn solve<W: std::io::Write>(scan: &[u8], out: &mut W) {
    let mut writer = CustomBufWriter::new(out);
    let pos = scan.iter().position(|byte| *byte <= b' ').unwrap();

    let children = u32::to_posint(&scan[..pos]);
    let mut circle: IndexSet<u32> = (1..(children + 1)).collect();
    let skip_amount = usize::to_posint(&scan[pos + 1..scan.len() - 1]);
    let mut index = skip_amount + 1;

    while circle.len() > 1 {
        index %= circle.len();
        if index == 0 {
            index = circle.len();
        }

        writer.add_int(circle.pop_index(index - 1));
        writer.add_byte(b' ');
        writer.maybe_flush(7);

        index += skip_amount;
    }

    writer.add_int(*circle.first().unwrap());
    writer.add_byte(b'\n');
}

// entrypoints //

fn main() {
    let mut buf_str = vec![];
    std::io::stdin().lock().read_to_end(&mut buf_str).unwrap();
    let mut out = std::io::stdout().lock();
    solve(&buf_str, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let mut out = Vec::with_capacity(target.len());
        solve(input, &mut out);

        assert_eq!(out, target);
    }

    #[test]
    fn test_example() {
        let input = b"\
7 2
";
        let target = b"\
3 6 2 7 5 1 4
";

        test(input, target);
    }

    #[test]
    fn test_example_2() {
        let input = b"\
12 1
";
        let target = b"\
2 4 6 8 10 12 3 7 11 5 1 9
";

        test(input, target);
    }

    #[test]
    fn test_no_skip() {
        let input = b"\
7 0
";
        let target = b"\
1 2 3 4 5 6 7
";

        test(input, target);
    }

    #[test]
    fn test_many_skip() {
        let input = b"\
7 5
";
        let target = b"\
6 5 7 2 1 4 3
";

        test(input, target);
    }

    #[test]
    fn test_large_skip() {
        let input = b"\
7 1000000000
";
        let target = b"\
7 5 6 1 3 4 2
";

        test(input, target);
    }
}
