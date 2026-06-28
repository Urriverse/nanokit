pub macro panic_handler() {
    #[panic_handler]
    fn ___km_ph(x: &core::panic::PanicInfo) -> ! {
        crate::sym::KeExecPanic(x)
    }
}

pub macro exit {
    () => {
        crate::keExecExit(0)
    },
    ($code:expr) => {
        crate::keExecExit($code)
    },
}
