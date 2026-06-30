use ketypes::*;

use crate::*;

#[repr(C, align(128))] pub struct Device; // ensure alignment matches with definition in kernel (for SOA, size == layout always)

impl Device {
    #[inline(always)]
    pub fn new(name: KeStr) -> Option<Box![Self]> { VtDeviceNew(name) }
}
