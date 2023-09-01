use crate::iter_set::{Iter, OwningIter};
#[cfg(feature = "raw-api")]
use crate::lock::RwLock;
use crate::setref::one::Ref;
use crate::DashMap;
#[cfg(feature = "raw-api")]
use crate::HashMap;
use cfg_if::cfg_if;
use core::borrow::Borrow;
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FromIterator;
use num_traits::{One, Zero};
use std::collections::hash_map::RandomState;
use std::iter;
/// `DashCounter` is a thin wrapper around [`DashMap`] to implement a counter.
/// It uses methods and types which are more convenient to work with on a Counter.
/// Mathematically, it is closer to a [Multiset](https://en.wikipedia.org/wiki/Multiset)
///
/// [`DashMap`]: struct.DashMap.html
type CounterMap<T, N> = DashMap<T, N>;

#[derive(Clone, Debug)]
pub struct DashCounter<T: Hash + Eq, N = usize> {
    map: CounterMap<T, N>,
    zero: N,
}