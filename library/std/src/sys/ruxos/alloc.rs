use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr;
use crate::sys::ruxos::abi;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        abi::sys_alloc(layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let addr = abi::sys_alloc(layout);

        if !addr.is_null() {
            ptr::write_bytes(addr, 0x00, layout.size());
        }

        addr
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        abi::sys_dealloc(ptr, layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        abi::sys_realloc(ptr, layout, new_size)
    }
}
