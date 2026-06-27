#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub(crate) extern "C" fn _start(st: crate::kst::KeSysTab) {
    (st.suicide)()
}
