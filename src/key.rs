//! Contains the [`Key`] trait and a default implementation.
//! It is used to identify entries in an arena.

use core::{
	num::NonZeroU32,
	ops::{Index, IndexMut},
};

use crate::version::Version;

/// A key for an entry in an arena. It can be implemented to allow
/// for better type safety finer control over the versioning strategy.
pub trait Key: Copy {
	type Version: Version;

	/// Attempts to construct a new key from an index and a version.
	/// This may fail if the index is too large for the underlying type.
	fn new(index: usize, version: Self::Version) -> Option<Self>;

	/// The index of the key.
	fn index(self) -> usize;

	/// The version of the key.
	fn version(self) -> Self::Version;
}

/// A well rounded key type that can be used in most situations.
/// It is a 32-bit unsigned integer with a generic versioning strategy.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Id<G = NonZeroU32> {
	index: u32,
	version: G,
}

impl<G: Version> Key for Id<G> {
	type Version = G;

	fn new(index: usize, version: Self::Version) -> Option<Self> {
		index.try_into().ok().map(|index| Self { index, version })
	}

	fn index(self) -> usize {
		self.index.try_into().unwrap()
	}

	fn version(self) -> Self::Version {
		self.version
	}
}

impl<G: Version> Default for Id<G> {
	fn default() -> Self {
		let index = u32::MAX;
		let version = G::new();

		Self { index, version }
	}
}

impl<G> core::fmt::Display for Id<G> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "I{}", self.index)
	}
}

impl<G: Version, T> Index<Id<G>> for [T] {
	type Output = T;

	#[inline]
	fn index(&self, id: Id<G>) -> &Self::Output {
		Index::index(self, id.index())
	}
}

impl<G: Version, T> IndexMut<Id<G>> for [T] {
	#[inline]
	fn index_mut(&mut self, id: Id<G>) -> &mut Self::Output {
		IndexMut::index_mut(self, id.index())
	}
}

impl<G: Version, T> Index<Id<G>> for alloc::vec::Vec<T> {
	type Output = T;

	#[inline]
	fn index(&self, id: Id<G>) -> &Self::Output {
		Index::index(self, id.index())
	}
}

impl<G: Version, T> IndexMut<Id<G>> for alloc::vec::Vec<T> {
	#[inline]
	fn index_mut(&mut self, id: Id<G>) -> &mut Self::Output {
		IndexMut::index_mut(self, id.index())
	}
}
