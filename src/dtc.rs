use std::ffi::CString;

extern "C" {
    fn dtc_main(argc: core::ffi::c_int, argv: *const *const core::ffi::c_char) -> core::ffi::c_int;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // convert args to a standard C-style `argc, argv`
    let args_c: Vec<CString> = args.iter().map(|arg| CString::new(arg.as_str()).unwrap()).collect();
    let args_c: Vec<*const i8> = args_c.iter().map(|arg| arg.as_ptr()).collect();

    unsafe { dtc_main(args_c.len() as _, args_c.as_slice().as_ptr() as _) };
}
