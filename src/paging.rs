#![allow(dead_code)]

use core::ops::{Index, IndexMut};

const PAGE_OFFSET_BITS: i32 = 12;
const PAGE_SIZE: usize = 1 << PAGE_OFFSET_BITS;
const PAGE_TABLE_ENTRY_COUNT: usize = 1 << 9;
const VIRTUAL_PAGE_NUMBER_BITS: i32 = 27;
const VIRTUAL_ADDRESS_BITS: i32 = VIRTUAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;
const PHYSICAL_PAGE_NUMBER_BITS: i32 = 44;
const PHYSICAL_ADDRESS_BITS: i32 = PHYSICAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PhysicalPageNumber(u64);

#[derive(Debug)]
pub struct InvalidPhysicalPageNumberError(u64);

impl TryFrom<u64> for PhysicalPageNumber {
    type Error = InvalidPhysicalPageNumberError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < 1 << PHYSICAL_PAGE_NUMBER_BITS {
            Ok(Self(value))
        } else {
            Err(InvalidPhysicalPageNumberError(value))
        }
    }
}

impl From<PhysicalPageNumber> for u64 {
    fn from(value: PhysicalPageNumber) -> Self {
        value.0
    }
}

pub enum PagePermissions {
    Read,
    ReadWrite,
    Execute,
    ReadExecute,
    ReadWriteExecute,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    const VALID_BIT: u64 = 1 << 0;
    const READABLE_BIT: u64 = 1 << 1;
    const WRITABLE_BIT: u64 = 1 << 2;
    const EXECUTABLE_BIT: u64 = 1 << 3;
    const USER_BIT: u64 = 1 << 4;
    const GLOBAL_BIT: u64 = 1 << 5;
    const ACCESSED_BIT: u64 = 1 << 6;
    const DIRTY_BIT: u64 = 1 << 7;
    const PPN_OFFSET: i32 = 10;

    pub const fn new_invalid() -> Self {
        Self(0)
    }

    pub fn new_leaf(
        ppn: PhysicalPageNumber,
        permissions: PagePermissions,
        user: bool,
        global: bool,
    ) -> Self {
        use PagePermissions::*;
        let mut bits = u64::from(ppn) << Self::PPN_OFFSET
            | Self::VALID_BIT
            | Self::ACCESSED_BIT
            | Self::DIRTY_BIT;
        bits |= match permissions {
            Read => Self::READABLE_BIT,
            ReadWrite => Self::READABLE_BIT | Self::WRITABLE_BIT,
            Execute => Self::EXECUTABLE_BIT,
            ReadExecute => Self::READABLE_BIT | Self::EXECUTABLE_BIT,
            ReadWriteExecute => Self::READABLE_BIT | Self::WRITABLE_BIT | Self::EXECUTABLE_BIT,
        };
        if user {
            bits |= Self::USER_BIT;
        }
        if global {
            bits |= Self::GLOBAL_BIT;
        }
        Self(bits)
    }
    pub fn new_non_leaf(ppn: PhysicalPageNumber, global: bool) -> Self {
        let mut bits = u64::from(ppn) << Self::PPN_OFFSET
            | Self::VALID_BIT
            | Self::ACCESSED_BIT
            | Self::DIRTY_BIT;
        if global {
            bits |= Self::GLOBAL_BIT;
        }
        Self(bits)
    }

    pub fn is_valid(self) -> bool {
        self.0 & Self::VALID_BIT != 0
    }

    pub fn physical_page_number(self) -> PhysicalPageNumber {
        PhysicalPageNumber::try_from(
            (self.0 >> Self::PPN_OFFSET) & ((1 << PHYSICAL_PAGE_NUMBER_BITS) - 1),
        )
        .unwrap()
    }

    pub fn is_leaf(self) -> bool {
        self.0 & (Self::READABLE_BIT | Self::WRITABLE_BIT | Self::EXECUTABLE_BIT) != 0
    }

    pub fn is_readable(self) -> bool {
        self.0 & Self::READABLE_BIT != 0
    }

    pub fn is_writable(self) -> bool {
        self.0 & Self::WRITABLE_BIT != 0
    }

    pub fn is_executable(self) -> bool {
        self.0 & Self::EXECUTABLE_BIT != 0
    }

    pub fn is_user(self) -> bool {
        self.0 & Self::USER_BIT != 0
    }

    pub fn is_global(self) -> bool {
        self.0 & Self::GLOBAL_BIT != 0
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

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
