use std::ops::{Index, IndexMut};

pub type ArenaEntryIndex = usize;

#[derive(Debug)]
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

impl<T> Index<&ArenaEntryIndex> for Arena<T> {
    type Output = ArenaEntry<T>;

    fn index(&self, arena_id: &ArenaEntryIndex) -> &Self::Output {
        &self.entries[*arena_id]
    }
}

impl<T> IndexMut<&ArenaEntryIndex> for Arena<T> {
    fn index_mut<'a>(&'a mut self, arena_id: &ArenaEntryIndex) -> &'a mut ArenaEntry<T> {
        &mut self.entries[*arena_id]
    }
}

impl<T> Arena<T> {
    pub fn push(&mut self, element: T) -> ArenaEntryIndex {
        let entry_index = self.offset.clone();
        self.entries
            .insert(self.offset, ArenaEntry::Occupied(element));
        self.offset += 1;
        entry_index
    }
    pub fn with_capacity(capacity: usize) -> Arena<T> {
        Arena {
            entries: Vec::with_capacity(capacity),
            len: capacity,
            offset: 0,
        }
    }

    pub fn new() -> Arena<T> {
        Arena {
            entries: Vec::new(),
            len: 0,
            offset: 0,
        }
    }
}
