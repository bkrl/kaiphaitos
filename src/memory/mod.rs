//! Memory management.

#![allow(dead_code)]

mod address;
mod frame_alloc;
mod page_table;

const PAGE_OFFSET_BITS: u32 = 12;
const PAGE_SIZE: usize = 1 << PAGE_OFFSET_BITS;
const VIRTUAL_PAGE_NUMBER_BITS: u32 = 27;
const VIRTUAL_ADDRESS_BITS: u32 = VIRTUAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;
const PHYSICAL_PAGE_NUMBER_BITS: u32 = 44;
const PHYSICAL_ADDRESS_BITS: u32 = PHYSICAL_PAGE_NUMBER_BITS + PAGE_OFFSET_BITS;
const PHYSICAL_MAPPING_OFFSET: u64 = 0xffffffff00000000;
