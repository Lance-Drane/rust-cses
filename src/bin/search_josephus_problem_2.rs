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

        self.range_idx(start_idx..=end_idx)
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
            Some(self.inner[back_node_idx].inner[..=back_start_idx].iter())
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

pub struct Cursor<'a, T>
where
    T: Ord + Clone,
{
    set: &'a IndexSet<T>,
    idx: usize,
}

impl<'a, T: Ord + Clone> Cursor<'a, T> {
    pub fn move_next(&mut self) {
        if self.idx == self.set.len() {
            self.idx = 0;
        } else {
            self.idx += 1;
        }
    }
    pub fn move_index(&mut self, index: usize) {
        self.idx = index;
    }
    pub fn move_prev(&mut self) {
        if self.idx == 0 {
            self.idx = self.set.len();
        } else {
            self.idx -= 1;
        }
    }
    pub fn item(&self) -> Option<&'a T> {
        return self.set.get_index(self.idx);
    }
    pub fn peek_next(&self) -> Option<&'a T> {
        if self.idx == self.set.len() {
            return self.set.first();
        }

        return self.set.get_index(self.idx + 1);
    }
    pub fn peek_index(&self, index: usize) -> Option<&'a T> {
        return self.set.get_index(index);
    }
    pub fn peek_prev(&self) -> Option<&'a T> {
        if self.idx == 0 {
            return None;
        }

        return self.set.get_index(self.idx - 1);
    }
}

// I/O boilerplate //

pub struct UnsafeScanner<'a> {
    // not actually dead code, needed for buf_iter to work
    #[allow(dead_code)]
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'a>,
}

impl UnsafeScanner<'_> {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        let mut buf_str = vec![];
        unsafe {
            reader.read_to_end(&mut buf_str).unwrap_unchecked();
        }
        let buf_iter = unsafe {
            let slice = std::str::from_utf8_unchecked(&buf_str);
            std::mem::transmute::<std::str::SplitAsciiWhitespace<'_>, std::str::SplitAsciiWhitespace<'_>>(slice.split_ascii_whitespace())
        };

        Self { buf_str, buf_iter }
    }

    /// Use "turbofish" syntax `token::<T>()` to select data type of next token.
    ///
    /// # Panics
    /// Panics if there's no more tokens or if the token cannot be parsed as T.
    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        unsafe {
            self.buf_iter
                .next()
                .unwrap_unchecked()
                .parse()
                .unwrap_unchecked()
        }
    }
}

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
fn solve<W: std::io::Write>(mut scan: UnsafeScanner, out: &mut W) {
    let children: u32 = scan.token();
    let mut circle: IndexSet<u32> = (1..=children).collect();
    let skip_amount = scan.token::<usize>();
    let mut index = skip_amount + 1;

    while circle.len() > 1 {
        index %= circle.len();
        if index == 0 {
            index = circle.len();
        }

        write!(out, "{} ", circle.pop_index(index - 1)).unwrap();

        index += skip_amount;
    }

    writeln!(out, "{}", circle.first().unwrap()).unwrap();
}

// entrypoints //

fn main() {
    let scan = UnsafeScanner::new(std::io::stdin());
    let mut out = std::io::BufWriter::with_capacity(32_768, std::io::stdout().lock());
    solve(scan, &mut out);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(input: &[u8], target: &[u8]) {
        let scan = UnsafeScanner::new(input);
        let mut out = Vec::with_capacity(target.len());
        solve(scan, &mut out);

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
