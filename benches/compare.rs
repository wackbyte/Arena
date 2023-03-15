use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use helper::BenchmarkGroupExt;

#[path = "support/helper.rs"]
mod helper;

fn bench_create_empty(c: &mut Criterion) {
	let mut group = c.benchmark_group("create_empty");
	for count in [1, 100, 10000] {
		group.throughput(Throughput::Elements(count as u64));
		group.bench("arena::Arena<Id>", count, || {
			use arena::{collection::Arena, key::Id};
			for _ in 0..count {
				black_box(Arena::<Id, usize>::new());
			}
		});
		group.bench("arena::Arena<Id<Nil>>", count, || {
			use arena::{collection::Arena, key::Id, version::Nil};
			for _ in 0..count {
				black_box(Arena::<Id<Nil>, usize>::new());
			}
		});
		group.bench("generational_arena::Arena", count, || {
			use generational_arena::Arena;
			for _ in 0..count {
				// `Arena::new` allocates a little by default.
				black_box(Arena::<usize>::with_capacity(0));
			}
		});
		group.bench("id_arena::Arena", count, || {
			use id_arena::Arena;
			for _ in 0..count {
				black_box(Arena::<usize>::new());
			}
		});
		group.bench("slab::Slab", count, || {
			use slab::Slab;
			for _ in 0..count {
				black_box(Slab::<usize>::new());
			}
		});
		group.bench("slotmap::SlotMap", count, || {
			use slotmap::SlotMap;
			for _ in 0..count {
				black_box(SlotMap::<_, usize>::new());
			}
		});
		group.bench("slotmap::DenseSlotMap", count, || {
			use slotmap::DenseSlotMap;
			for _ in 0..count {
				black_box(DenseSlotMap::<_, usize>::new());
			}
		});
		group.bench("slotmap::HopSlotMap", count, || {
			use slotmap::HopSlotMap;
			for _ in 0..count {
				black_box(HopSlotMap::<_, usize>::new());
			}
		});
		group.bench("thunderdome::Arena", count, || {
			use thunderdome::Arena;
			for _ in 0..count {
				black_box(Arena::<usize>::new());
			}
		});
	}
	group.finish();
}

fn bench_insert(c: &mut Criterion) {
	let mut group = c.benchmark_group("insert");
	for count in [1, 100, 10000] {
		group.throughput(Throughput::Elements(count as u64));
		group.batch(
			"arena::Arena<Id>",
			count,
			|| {
				use arena::{collection::Arena, key::Id};
				Arena::<Id, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					let _ = arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"arena::Arena<Id<Nil>>",
			count,
			|| {
				use arena::{collection::Arena, key::Id, version::Nil};
				Arena::<Id<Nil>, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					let _ = arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"generational_arena::Arena",
			count,
			|| {
				use generational_arena::Arena;
				Arena::<usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"id_arena::Arena",
			count,
			|| {
				use id_arena::Arena;
				Arena::<usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.alloc(black_box(0));
				}
			},
		);
		group.batch(
			"slab::Slab",
			count,
			|| {
				use slab::Slab;
				Slab::<usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"slotmap::SlotMap",
			count,
			|| {
				use slotmap::SlotMap;
				SlotMap::<_, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"slotmap::DenseSlotMap",
			count,
			|| {
				use slotmap::DenseSlotMap;
				DenseSlotMap::<_, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"slotmap::HopSlotMap",
			count,
			|| {
				use slotmap::HopSlotMap;
				HopSlotMap::<_, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"thunderdome::Arena",
			count,
			|| {
				use thunderdome::Arena;
				Arena::<usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
	}
	group.finish();
}

fn bench_insert_and_remove(c: &mut Criterion) {
	let mut group = c.benchmark_group("insert_and_remove");
	for count in [1, 100, 10000] {
		group.throughput(Throughput::Elements(count as u64));
		group.batch(
			"arena::Arena<Id>",
			count,
			|| {
				use arena::{collection::Arena, key::Id};
				Arena::<Id, usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
		group.batch(
			"arena::Arena<Id<Nil>>",
			count,
			|| {
				use arena::{collection::Arena, key::Id, version::Nil};
				Arena::<Id<Nil>, usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
		group.batch(
			"generational_arena::Arena",
			count,
			|| {
				use generational_arena::Arena;
				Arena::<usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
		group.batch(
			"slab::Slab",
			count,
			|| {
				use slab::Slab;
				Slab::<usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
		group.batch(
			"slotmap::SlotMap",
			count,
			|| {
				use slotmap::SlotMap;
				SlotMap::<_, usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
		group.batch(
			"slotmap::DenseSlotMap",
			count,
			|| {
				use slotmap::DenseSlotMap;
				DenseSlotMap::<_, usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
		group.batch(
			"slotmap::HopSlotMap",
			count,
			|| {
				use slotmap::HopSlotMap;
				HopSlotMap::<_, usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
		group.batch(
			"thunderdome::Arena",
			count,
			|| {
				use thunderdome::Arena;
				Arena::<usize>::with_capacity(1)
			},
			|mut arena| {
				for _ in 0..count {
					let key = arena.insert(black_box(0));
					arena.remove(black_box(key));
				}
			},
		);
	}
	group.finish();
}

fn bench_insert_impls(c: &mut Criterion) {
	let mut group = c.benchmark_group("insert_impls");
	for count in [1, 100, 10000] {
		group.throughput(Throughput::Elements(count as u64));
		group.batch(
			"arena::Arena<Id>::insert",
			count,
			|| {
				use arena::{collection::Arena, key::Id};
				Arena::<Id, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"arena::Arena<Id<Nil>>::insert",
			count,
			|| {
				use arena::{collection::Arena, key::Id, version::Nil};
				Arena::<Id<Nil>, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert(black_box(0));
				}
			},
		);
		group.batch(
			"arena::Arena<Id>::insert_alt",
			count,
			|| {
				use arena::{collection::Arena, key::Id};
				Arena::<Id, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert_alt(black_box(0));
				}
			},
		);
		group.batch(
			"arena::Arena<Id<Nil>>::insert_alt",
			count,
			|| {
				use arena::{collection::Arena, key::Id, version::Nil};
				Arena::<Id<Nil>, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					arena.insert_alt(black_box(0));
				}
			},
		);
		group.batch(
			"arena::Arena<Id>::insert_orig",
			count,
			|| {
				use arena::{collection::Arena, key::Id};
				Arena::<Id, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					let _ = arena.insert_orig(black_box(0));
				}
			},
		);
		group.batch(
			"arena::Arena<Id<Nil>>::insert_orig",
			count,
			|| {
				use arena::{collection::Arena, key::Id, version::Nil};
				Arena::<Id<Nil>, usize>::with_capacity(count)
			},
			|mut arena| {
				for _ in 0..count {
					let _ = arena.insert_orig(black_box(0));
				}
			},
		);
	}
	group.finish();
}

criterion_group!(
	benches,
	bench_create_empty,
	bench_insert,
	bench_insert_and_remove,
	bench_insert_impls,
);
criterion_main!(benches);
