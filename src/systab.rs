use crate::KeInvoke;

#[repr(C)]
pub struct KeSysTab {
    pub log             :   fn(u8, &'static str, &'static str, u32, &core::fmt::Arguments) -> (),
    pub panic           :   fn(&core::panic::PanicInfo) -> !,
    pub alloc           :   fn(core::alloc::Layout) -> *mut u8,
    pub free            :   fn(*mut u8, core::alloc::Layout) -> (),
    pub exit            :   fn(i32) -> !,
}

pub struct KeSysTabPtr(pub *const KeSysTab);

unsafe impl Sync for KeSysTabPtr {}

#[inline]
pub fn exit(code: i32) -> ! { KeInvoke!(exit: code) }
