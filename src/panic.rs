#[cfg(not(test))]
#[panic_handler]
pub fn _panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
