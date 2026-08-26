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

    #[inline(always)]
    pub fn add_entry(&mut self, kind: EntryKind, index: u32) {
        self.entries.push(Entry { kind, index });
    }

    #[inline(always)]
    pub fn add_string_markers(&mut self, start: u32, end: u32) {
        self.entries.push(Entry {
            kind: EntryKind::StringStart,
            index: start,
        });
        self.entries.push(Entry {
            kind: EntryKind::StringEnd,
            index: end,
        });
    }

    #[inline(always)]
    pub fn add_number_markers(&mut self, start: u32, end: u32) {
        self.entries.push(Entry {
            kind: EntryKind::NumberStart,
            index: start,
        });
        self.entries.push(Entry {
            kind: EntryKind::NumberEnd,
            index: end,
        });
    }

    #[cfg(test)]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}
