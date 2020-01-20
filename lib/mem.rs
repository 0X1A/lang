use crate::error::*;
use crate::value::*;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};

pub type ArenaEntryIndex = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum ArenaEntry<T> {
    Emtpy,
    Occupied(T),
}

impl TryInto<TypedValue> for &ArenaEntry<TypedValue> {
    type Error = LangError;
    fn try_into(self) -> Result<TypedValue, Self::Error> {
        match self {
            ArenaEntry::Occupied(value) => Ok(value.clone()),
            ArenaEntry::Emtpy => Err(LangErrorType::new_iie_error(
                "tried to index an empty arena entry".into(),
            )),
        }
    }
}

impl TryInto<TypedValue> for ArenaEntry<TypedValue> {
    type Error = LangError;
    fn try_into(self) -> Result<TypedValue, Self::Error> {
        match self {
            ArenaEntry::Occupied(value) => Ok(value),
            ArenaEntry::Emtpy => Err(LangErrorType::new_iie_error(
                "tried to index an empty arena entry".into(),
            )),
        }
    }
}

impl<'a> TryInto<&'a TypedValue> for &'a ArenaEntry<TypedValue> {
    type Error = LangError;
    fn try_into(self) -> Result<&'a TypedValue, Self::Error> {
        match self {
            ArenaEntry::Occupied(ref value) => Ok(value),
            ArenaEntry::Emtpy => Err(LangErrorType::new_iie_error(
                "tried to index an empty arena entry".into(),
            )),
        }
    }
}

impl<'a> TryInto<&'a mut TypedValue> for &'a mut ArenaEntry<TypedValue> {
    type Error = LangError;
    fn try_into(self) -> Result<&'a mut TypedValue, Self::Error> {
        match self {
            ArenaEntry::Occupied(ref mut value) => Ok(value),
            ArenaEntry::Emtpy => Err(LangErrorType::new_iie_error(
                "tried to index an empty arena entry".into(),
            )),
        }
    }
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
    fn index_mut(&mut self, arena_id: ArenaEntryIndex) -> &mut ArenaEntry<T> {
        &mut self.entries[arena_id]
    }
}

impl<T: std::fmt::Debug> Arena<T> {
    pub fn insert(&mut self, element: T) -> ArenaEntryIndex {
        println!("Arena::insert {:?}", element);
        let arena_index = self.offset;
        self.offset += 1;
        self.entries
            .insert(arena_index, ArenaEntry::Occupied(element));
        self.len += 1;
        arena_index
    }

    pub fn get(&mut self, index: ArenaEntryIndex) -> ArenaEntry<T> {
        self.entries.remove(index)
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

    pub fn entries(&self) -> &Vec<ArenaEntry<T>> {
        &self.entries
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

    pub fn update_entry<Closure>(
        &mut self,
        index: ArenaEntryIndex,
        closure: Closure,
    ) -> Result<(), LangError>
    where
        Closure: FnOnce(&mut T) -> Result<(), LangError>,
    {
        let value_entry = &mut self.entries[index];
        let value: &mut T = match value_entry {
            ArenaEntry::Occupied(ref mut v) => v,
            _ => panic!(),
        };
        closure(value)?;
        Ok(())
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
