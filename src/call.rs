//! Helpers for managing call()s from BYOND code.
use std::ffi::{CString, CStr, NulError};
use std::slice;
use std::sync::Mutex;
use libc;

// Yes this will totally break if multithreading would happen, but won't so hooray!

// Can't use an on-stack value to give back to BYOND for obvious reasons.
lazy_static! {
    static ref BYOND_RETURN: Mutex<CString> = {
        Mutex::new(CString::new("hi!".as_bytes()).unwrap())
    };
}


/// Returns a pointer that can be returned to BYOND from a call()ed function, to return a string.
pub fn return_to_byond(string: &str) -> Result<*const libc::c_char, NulError> {
    let cstr = CString::new(string.as_bytes())?;

    let mut mutex = BYOND_RETURN.lock().unwrap();

    *mutex = cstr;
    Ok(mutex.as_ptr())
}


/// Turns the arguments supplied by BYOND into a more workable vector.
pub fn from_byond_args(n: libc::c_int, v: *const *const libc::c_schar) -> Vec<String> {
    let mut args = Vec::new();
    unsafe {
        let slice = slice::from_raw_parts(v, n as usize);
        for ptr in slice {
            let cstr = CStr::from_ptr(*ptr);
            let string = String::from_utf8_lossy(cstr.to_bytes()).into_owned();
            args.push(string);
        }
    }
    args
}
