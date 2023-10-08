//! Types for physical and virtual addresses.

#![allow(dead_code)]

pub const PAGE_OFFSET_BITS: i32 = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_OFFSET_BITS;
pub const VIRTUAL_PAGE_NUMBER_BITS: i32 = 27;
pub const VIRTUAL_ADDRESS_BITS: i32 = VIRTUAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;
pub const PHYSICAL_PAGE_NUMBER_BITS: i32 = 44;
pub const PHYSICAL_ADDRESS_BITS: i32 = PHYSICAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;

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
