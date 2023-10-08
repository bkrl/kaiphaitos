//! Types for physical and virtual addresses.

use super::{PAGE_OFFSET_BITS, PHYSICAL_ADDRESS_BITS};

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
