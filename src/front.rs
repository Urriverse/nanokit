use ketypes::*;

use crate::*;

#[repr(C, align(128))] pub struct KeDevice; // ensure alignment matches with definition in kernel (for SOA, size == layout always)

impl KeDevice {
    #[inline(always)]
    pub fn new(name: KeStr) -> Option<Box![Self]> { KeVtDeviceNew(name) }
}
