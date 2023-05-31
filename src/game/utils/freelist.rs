use rayon::iter::*;
use std::ops::{Index, IndexMut};

pub struct FreeList<T> {
	inner: Vec<Elem<T>>,
	free: Option<usize>,
}

impl<T> FreeList<T> {
	pub fn new() -> Self {
		Self {
			inner: vec![],
			free: None,
		}
	}

	pub fn insert(&mut self, item: T) -> usize {
		if let Some(first_free) = self.free {
			self.free = match self.inner[first_free] {
				Elem::NextFree(next) => Some(next),
				Elem::LastFree => None,
				Elem::Obj(_) => unreachable!(),
			};
			self.inner[first_free] = Elem::Obj(item);
			first_free
		} else {
			self.inner.push(Elem::Obj(item));
			self.inner.len() - 1
		}
	}

	pub fn remove(&mut self, index: usize) -> Option<T> {
		let obj = std::mem::replace(&mut self.inner[index], Elem::LastFree);

		if let Some(free) = self.free {
			self.inner[index] = Elem::NextFree(free);
		};

		self.free = Some(index);

		match obj {
			Elem::Obj(item) => Some(item),
			_ => None,
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.inner.iter().filter_map(|elem| match elem {
			Elem::Obj(item) => Some(item),
			_ => None,
		})
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.inner.iter_mut().filter_map(|elem| match elem {
			Elem::Obj(item) => Some(item),
			_ => None,
		})
	}
}

impl<T: Send + Sync> FreeList<T> {
	pub fn par_iter(&self) -> impl ParallelIterator<Item = &T> {
		self.inner.par_iter().filter_map(|elem| match elem {
			Elem::Obj(item) => Some(item),
			_ => None,
		})
	}

	pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T> {
		self.inner.par_iter_mut().filter_map(|elem| match elem {
			Elem::Obj(item) => Some(item),
			_ => None,
		})
	}
}

impl<T> Index<usize> for FreeList<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		match &self.inner[index] {
			Elem::Obj(item) => item,
			_ => panic!("Attempted to access empty freelist slot."),
		}
	}
}

impl<T> IndexMut<usize> for FreeList<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match &mut self.inner[index] {
			Elem::Obj(item) => item,
			_ => panic!("Attempted to access empty freelist slot."),
		}
	}
}

enum Elem<T> {
	Obj(T),
	NextFree(usize),
	LastFree,
}
