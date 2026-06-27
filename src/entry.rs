pub macro entry( $($b:tt)* ) {
    #[unsafe(no_mangle)]
    #[inline(never)]
    pub extern "C" fn module_start() {
        $($b)*
    }
}
