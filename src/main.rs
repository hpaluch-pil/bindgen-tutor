// src/main.rs - example how to call generated bzlib2 function wrapper from
//               our Rust code

use std::ffi::CStr;
use std::str::Utf8Error;

fn main() -> Result<(),Utf8Error> {
    // WARNING! We expect that BZ2_bzlibVersion() returns pointer
    //          to constant string that 
    //          has 'static lifetime (which const char * should)
    let bz_ver_ptr = unsafe { bindgen_ex::BZ2_bzlibVersion() };
    let bz_ver_cstr = unsafe { CStr::from_ptr(bz_ver_ptr) };
    let bz_ver_str = bz_ver_cstr.to_str()?;
    println!("bzip2 version is: '{}'", bz_ver_str);

    Ok(())
}
