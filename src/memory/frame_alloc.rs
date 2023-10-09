//! Frame allocator using a lazily-initialized free list.

use core::{mem, ptr};

use spin::Mutex;

use super::address::PhysicalPageAddress;
use super::{PAGE_SIZE, PHYSICAL_MAPPING_OFFSET};
use crate::static_assert::static_assert;

extern "C" {
    /// Dummy static used to get the address of the first allocatable frame, which is set in the
    /// linker script.
    // rustc seems to think that all tuples have unspecified layout, even though unit tuples are
    // guaranteed to have zero size. The annotation suppresses a warning about using non-FFI-safe
    // types in an extern block.
    #[allow(improper_ctypes)]
    static FIRST_ALLOCATABLE_FRAME: ();
}

#[repr(C, align(4096))]
struct FreeFrame {
    next: *mut FreeFrame,
}

static_assert!(mem::size_of::<FreeFrame>() == PAGE_SIZE);
static_assert!(mem::align_of::<FreeFrame>() == PAGE_SIZE);

struct FrameAllocator {
    free_list_head: *mut FreeFrame,
    /// Address of the first frame that hasn't been initialized.
    next_uninitialized_frame: *mut FreeFrame,
}

// The mutex below has to implement Sync because it is static, and the thing inside a Sync mutex
// has to implement Send. Send isn't automatically implemented because of the raw pointers.
unsafe impl Send for FrameAllocator {}

impl FrameAllocator {
    fn alloc(&mut self) -> *mut FreeFrame {
        unsafe {
            if self.free_list_head.is_null() {
                let frame = self.next_uninitialized_frame;
                self.next_uninitialized_frame = self.next_uninitialized_frame.offset(1);
                frame
            } else {
                let frame = self.free_list_head;
                self.free_list_head = (*self.free_list_head).next;
                frame
            }
        }
    }

    unsafe fn free(&mut self, frame: *mut FreeFrame) {
        unsafe {
            (*frame).next = self.free_list_head;
            self.free_list_head = frame;
        }
    }
}

static FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator {
    free_list_head: ptr::null_mut(),
    next_uninitialized_frame: unsafe { ptr::addr_of!(FIRST_ALLOCATABLE_FRAME) } as _,
});

/// Allocate a frame.
pub fn frame_alloc() -> PhysicalPageAddress {
    virtual_to_physical(FRAME_ALLOCATOR.lock().alloc())
}

/// Allocate a frame, returning both a physical address and a pointer that can be used to write to
/// the frame.
pub fn frame_alloc_with_ptr<T>() -> (PhysicalPageAddress, *mut T) {
    let frame = FRAME_ALLOCATOR.lock().alloc();
    (virtual_to_physical(frame), frame as _)
}

/// Free a frame. Freeing a frame that wasn't allocated will cause bad things, so this is unsafe.
pub unsafe fn frame_free(frame: PhysicalPageAddress) {
    FRAME_ALLOCATOR.lock().free(physical_to_virtual(frame))
}

fn virtual_to_physical<T>(frame: *mut T) -> PhysicalPageAddress {
    (frame as u64 - PHYSICAL_MAPPING_OFFSET).try_into().unwrap()
}

fn physical_to_virtual<T>(frame: PhysicalPageAddress) -> *mut T {
    (u64::from(frame) + PHYSICAL_MAPPING_OFFSET) as _
}
