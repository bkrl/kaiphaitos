//! Page table entries and page tables.

#![allow(dead_code)]

use core::mem::{align_of, size_of};
use core::ops::Index;

use bitfield::bitfield;

use super::PAGE_SIZE;
use crate::static_assert::static_assert;

const PAGE_TABLE_ENTRY_COUNT: usize = PAGE_SIZE / core::mem::size_of::<PageTableEntry>();

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PagePermissions {
    Read,
    ReadWrite,
    Execute,
    ReadExecute,
    ReadWriteExecute,
}

bitfield! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct PageTableEntry(u64);
    impl Debug;
    pub valid, set_valid: 0;
    // Permission bits are private since some combinations are reserved.  The bitfield crate
    // doesn't seem to support different visibility for getters and setters, so they are set to
    // private here and public wrappers are defined for the getters.
    readable_, set_readable: 1;
    writable_, set_writable: 2;
    executable_, set_executable: 3;
    pub user, set_user: 4;
    pub global, set_global: 5;
    pub accessed, set_accessed: 6;
    pub dirty, set_dirty: 7;
    pub u64, ppn, set_ppn: 10, 53;
}

impl PageTableEntry {
    pub const fn new_invalid() -> Self {
        Self(0)
    }

    pub fn readable(&self) -> bool {
        self.readable_()
    }

    pub fn writable(&self) -> bool {
        self.writable_()
    }

    pub fn executable(&self) -> bool {
        self.executable_()
    }
}

#[repr(C, align(4096))]
pub struct PageTable(pub [PageTableEntry; PAGE_TABLE_ENTRY_COUNT]);

static_assert!(size_of::<PageTable>() == PAGE_SIZE);
static_assert!(align_of::<PageTable>() == PAGE_SIZE);

impl PageTable {
    pub const fn new() -> Self {
        PageTable([PageTableEntry::new_invalid(); PAGE_TABLE_ENTRY_COUNT])
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
