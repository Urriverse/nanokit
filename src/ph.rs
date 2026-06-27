pub macro panic_handler() { #[panic_handler] pub fn phdl(pi: &core::panic::PanicInfo) -> ! { crate::KeInvoke!(panic: pi) } }
