use std::ops::{Index, IndexMut};

pub type ArenaEntryIndex = usize;

#[derive(Debug, PartialEq)]
pub enum ArenaEntry<T> {
    Emtpy,
    Occupied(T),
}

#[derive(Debug)]
pub struct Arena<T> {
    entries: Vec<ArenaEntry<T>>,
    len: usize,
    offset: usize,
}

impl<T> Index<ArenaEntryIndex> for Arena<T> {
    type Output = ArenaEntry<T>;

    fn index(&self, arena_id: ArenaEntryIndex) -> &Self::Output {
        &self.entries[arena_id]
    }
}

impl<T> IndexMut<ArenaEntryIndex> for Arena<T> {
    fn index_mut<'a>(&'a mut self, arena_id: ArenaEntryIndex) -> &'a mut ArenaEntry<T> {
        &mut self.entries[arena_id]
    }
}

impl<T> Arena<T> {
    pub fn insert(&mut self, element: T) -> ArenaEntryIndex {
        let arena_index = self.offset;
        self.offset += 1;
        self.entries
            .insert(arena_index, ArenaEntry::Occupied(element));
        arena_index
    }

    pub fn remove(&mut self, index: ArenaEntryIndex) {
        self.entries.insert(index, ArenaEntry::Emtpy)
    }

    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    pub fn with_capacity(capacity: usize) -> Arena<T> {
        Arena {
            entries: Vec::with_capacity(capacity),
            len: capacity,
            offset: 0,
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.entries.reserve_exact(capacity)
    }

    pub fn new() -> Arena<T> {
        Arena {
            entries: Vec::new(),
            len: 0,
            offset: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mem::*;

    #[test]
    fn arena_new_with_capacity() {
        let arena: Arena<()> = Arena::with_capacity(10);
        assert_eq!(arena.capacity(), 10);

        let arena_empty: Arena<()> = Arena::new();
        assert_eq!(arena_empty.capacity(), 0);
    }

    #[test]
    fn insert() {
        let mut arena: Arena<()> = Arena::with_capacity(0);
        arena.reserve(10);
        for _ in 0..3 {
            let index = arena.insert(());
            assert_eq!(arena[index], ArenaEntry::Occupied(()))
        }
        println!("{:?}", arena);
    }
}
