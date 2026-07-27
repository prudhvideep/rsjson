#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EntryKind {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    StringStart,
    StringEnd,
    NumberStart,
    NumberEnd,
    True,
    False,
    Null,
}

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub kind: EntryKind,
    pub index: u32,
}

#[derive(Debug)]
pub struct Tape {
    entries: Vec<Entry>,
}

impl Tape {
    pub fn with_capacity(c: usize) -> Self {
        Self {
            entries: Vec::with_capacity(c),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    #[cfg(test)]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}
