use crate::sync::atomic::AtomicU32;
use crate::time::Duration;
use crate::sys::ruxos::abi;

pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    unsafe { abi::sys_futex_wait(futex, expected, timeout) }
}

#[inline]
pub fn futex_wake(futex: &AtomicU32) -> bool {
	unsafe { abi::sys_futex_wake(futex, 1) };
    true
}

#[inline]
pub fn futex_wake_all(futex: &AtomicU32) {
	unsafe { abi::sys_futex_wake(futex, i32::MAX) };
}
