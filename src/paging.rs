//! Paging support.

// Suppress dead code warnings during development.
#![allow(dead_code)]

use bitfield::bitfield;
use core::ops::Index;

const PAGE_OFFSET_BITS: i32 = 12;
const PAGE_SIZE: usize = 1 << PAGE_OFFSET_BITS;
const PAGE_TABLE_ENTRY_COUNT: usize = 1 << 9;
const VIRTUAL_PAGE_NUMBER_BITS: i32 = 27;
const VIRTUAL_ADDRESS_BITS: i32 = VIRTUAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;
const PHYSICAL_PAGE_NUMBER_BITS: i32 = 44;
const PHYSICAL_ADDRESS_BITS: i32 = PHYSICAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;

#[derive(Debug)]
pub struct InvalidAddressError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PhysicalAddress(u64);

impl TryFrom<u64> for PhysicalAddress {
    type Error = InvalidAddressError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < 1 << PHYSICAL_ADDRESS_BITS {
            Ok(Self(value))
        } else {
            Err(InvalidAddressError)
        }
    }
}

impl From<PhysicalAddress> for u64 {
    fn from(value: PhysicalAddress) -> Self {
        value.0
    }
}

impl PhysicalAddress {
    pub fn page_offset(self) -> u64 {
        self.0 & ((1 << PAGE_OFFSET_BITS) - 1)
    }
    pub fn is_page_aligned(self) -> bool {
        self.page_offset() == 0
    }
}

/// A page-aligned physical address.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PhysicalPageAddress(PhysicalAddress);

impl TryFrom<PhysicalAddress> for PhysicalPageAddress {
    type Error = InvalidAddressError;

    fn try_from(addr: PhysicalAddress) -> Result<Self, Self::Error> {
        if addr.is_page_aligned() {
            Ok(Self(addr))
        } else {
            Err(InvalidAddressError)
        }
    }
}

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
    // Permission bits are private since some combinations are reserved.
    // The bitfield crate doesn't seem to support different visibility for getters and setters,
    // so they are set to private here and public wrappers are defined for the getters.
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
