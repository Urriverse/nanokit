use alloc::boxed::Box;

#[repr(C, align(128))] pub struct Device;

ketypes::Import!(fn VtDeviceNew(name: &str) -> Option<Box<Device>> where kernel 0.1);

impl Device {
    #[inline(always)]
    pub fn new(name: &str) -> Option<Box<Self>> { VtDeviceNew(name) }
}
