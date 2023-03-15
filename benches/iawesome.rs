use core::hint::black_box;

use arena::{collection::Arena, key::Id, version::Nil};

const ARENA: Arena<Id, usize> = Arena::new();
const ARENA_NIL: Arena<Id<Nil>, usize> = Arena::new();
const ARENA_ID: Arena<Id, Id> = Arena::new();
const ARENA_ID_NIL: Arena<Id<Nil>, Id<Nil>> = Arena::new();

fn insert() {
	let mut arena = ARENA;
	arena.insert(black_box(0));
}

fn insert_nil() {
	let mut arena = ARENA_NIL;
	arena.insert(black_box(0));
}

fn insert_alt() {
	let mut arena = ARENA;
	arena.insert_alt(black_box(0));
}

fn insert_alt_nil() {
	let mut arena = ARENA_NIL;
	arena.insert_alt(black_box(0));
}

fn insert_orig() {
	let mut arena = ARENA;
	let _ = arena.insert_orig(black_box(0));
}

fn insert_orig_nil() {
	let mut arena = ARENA_NIL;
	let _ = arena.insert_orig(black_box(0));
}

fn insert_with() {
	let mut arena = ARENA_ID;
	arena.insert_with(black_box(|key| key));
}

fn insert_with_nil() {
	let mut arena = ARENA_ID_NIL;
	arena.insert_with(black_box(|key| key));
}

fn insert_with_alt() {
	let mut arena = ARENA_ID;
	arena.insert_with_alt(black_box(|key| key));
}

fn insert_with_alt_nil() {
	let mut arena = ARENA_ID_NIL;
	arena.insert_with_alt(black_box(|key| key));
}

iai::main!(
	insert,
	insert_nil,
	insert_alt,
	insert_alt_nil,
	insert_orig,
	insert_orig_nil,
	insert_with,
	insert_with_nil,
	insert_with_alt,
	insert_with_alt_nil,
);
