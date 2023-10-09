//! Types for physical and virtual addresses.

use core::fmt;

use super::{PAGE_OFFSET_BITS, PHYSICAL_ADDRESS_BITS};

#[derive(Debug)]
pub struct InvalidAddressError;

#[derive(PartialEq, Eq, Clone, Copy)]
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

impl fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
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
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct PhysicalPageAddress(PhysicalAddress);

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

impl TryFrom<u64> for PhysicalPageAddress {
    type Error = InvalidAddressError;

    fn try_from(val: u64) -> Result<Self, Self::Error> {
        PhysicalAddress::try_from(val)?.try_into()
    }
}

impl From<PhysicalPageAddress> for u64 {
    fn from(value: PhysicalPageAddress) -> Self {
        value.0.into()
    }
}

impl fmt::Debug for PhysicalPageAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
