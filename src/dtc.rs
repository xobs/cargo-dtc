use std::ffi::CString;

extern "C" {
    fn dtc_main(argc: core::ffi::c_int, argv: *const *const core::ffi::c_char) -> core::ffi::c_int;
}

fn dtc(args: &[&str]) -> usize {
    // convert args to a standard C-style `argc, argv`
    let args_c: Vec<CString> = args.iter().map(|arg| CString::new(*arg).unwrap()).collect();
    let args_c: Vec<*const i8> = args_c.iter().map(|arg| arg.as_ptr()).collect();

    unsafe { dtc_main(args_c.len() as _, args_c.as_slice().as_ptr() as _) as _ }
}

fn main() -> Result<(), usize> {
    let string_args = std::env::args().collect::<Vec<String>>();
    let mut str_args: Vec<&str> = vec![];
    for arg in &string_args {
        str_args.push(arg);
    }

    // If this command is run as `cargo dtc`, then the first argument is `cargo`.
    // Strip it off so that the first argument is `dtc`.
    if str_args.get(1).map(|x| x == &"dtc").unwrap_or(false) {
        str_args.remove(0);
    }

    let result = dtc(&str_args);
    if result == 0 {
        Ok(())
    } else {
        Err(result)
    }
}

#[test]
fn test_help() {
    dtc(&["dtc", "--help"]);
}
