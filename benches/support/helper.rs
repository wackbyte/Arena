use std::fmt::Display;

use criterion::{measurement::Measurement, BatchSize, BenchmarkGroup, BenchmarkId};

pub trait BenchmarkGroupExt {
	fn bench<O>(
		&mut self,
		name: impl Into<String>,
		parameter: impl Display,
		routine: impl FnMut() -> O,
	) -> &mut Self;

	fn batch<I, O>(
		&mut self,
		name: impl Into<String>,
		parameter: impl Display,
		setup: impl FnMut() -> I,
		routine: impl FnMut(I) -> O,
	) -> &mut Self;
}

impl<'a, M: Measurement> BenchmarkGroupExt for BenchmarkGroup<'a, M> {
	fn bench<O>(
		&mut self,
		name: impl Into<String>,
		parameter: impl Display,
		mut routine: impl FnMut() -> O,
	) -> &mut Self {
		self.bench_function(BenchmarkId::new(name, parameter), |b| {
			b.iter(&mut routine);
		})
	}

	fn batch<I, O>(
		&mut self,
		name: impl Into<String>,
		parameter: impl Display,
		mut setup: impl FnMut() -> I,
		mut routine: impl FnMut(I) -> O,
	) -> &mut Self {
		self.bench_function(BenchmarkId::new(name, parameter), |b| {
			b.iter_batched(&mut setup, &mut routine, BatchSize::SmallInput);
		})
	}
}
