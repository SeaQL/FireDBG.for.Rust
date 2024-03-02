use std::ops::{Deref, Range};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRange(Range<usize>);

impl Deref for PageRange {
    type Target = Range<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PageRange {
    pub fn new(start: usize, end: usize) -> PageRange {
        PageRange(start..end)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pages: PageRange,
}

impl Block {
    pub fn new(start: usize, len: usize) -> Block {
        let pages = PageRange::new(start, start + len);
        Block { pages }
    }

    pub fn start(&self) -> usize {
        self.pages.start
    }

    pub fn end(&self) -> usize {
        self.pages.end
    }

    pub fn pages(&self) -> &PageRange {
        &self.pages
    }
}

fn mk_block(start: usize, len: usize) -> Block {
    Block::new(start, len)
}

fn main() {
    let block = mk_block(0, 1);
    assert_eq!(block.start(), 0);
    assert_eq!(block.end(), 1);
    assert_eq!(block.pages(), &PageRange::new(0, 1));
}