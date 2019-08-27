//! Helpers for managing `call()()`s from BYOND code.
use std::cell::RefCell;
use std::ffi::{CStr, CString, NulError};
use std::os::raw::{c_char, c_int};
use std::ptr::null;
use std::slice;

thread_local! {
    static BYOND_RETURN: RefCell<Option<CString>> = {
        RefCell::new(None)
    };
}

/// Returns a pointer that can be returned to BYOND from a `call()()`ed function,
/// to return a string.
pub fn return_to_byond<T: AsRef<[u8]>>(string: T) -> Result<*const c_char, NulError> {
    let cstr = CString::new(string.as_ref())?;
    let ptr = cstr.as_ptr();

    BYOND_RETURN.with(|f| {
        *f.borrow_mut() = Some(cstr);
    });

    Ok(ptr)
}

/// Turns the arguments supplied by BYOND into a more workable vector.
///
/// All strings are converted into UTF-8 losilly. You've been warned.
pub unsafe fn from_byond_args(n: c_int, v: *const *const c_char) -> Vec<String> {
    let mut args = Vec::new();
    let slice = slice::from_raw_parts(v, n as usize);
    for ptr in slice {
        let cstr = CStr::from_ptr(*ptr);
        let string = String::from_utf8_lossy(cstr.to_bytes()).into_owned();
        args.push(string);
    }
    args
}

/// Allows one to easily test BYOND callbacks.
/// Does *not* take arguments to pass down.
///
/// Warning: This passes a straight nullptr to the function's argument list.
/// As such, if the function expects any arguments, this will probably segfault.
/// Non-UTF-8 returned strings are lossily converted.
/// # Panics
/// Panics if the strings in args contain a NUL byte.
pub fn test_byond_call(
    func: unsafe extern "C" fn(i32, *const *const c_char) -> *const c_char,
) -> String {
    unsafe {
        let ptr = func(0, null());
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

/// Allows one to easily test BYOND callbacks.
/// Takes arguments that are passed down to the function.
///
/// # Panics
/// Panics if the strings in args contain a NUL byte.
/// Non-UTF-8 strings are lossily converted.
pub fn test_byond_call_args<P>(
    func: unsafe extern "C" fn(i32, *const *const c_char) -> *const c_char,
    args: &[P],
) -> String
where
    P: AsRef<[u8]>,
{
    // Need to keep track of the CStrs so they dont Drop.
    let mut cstrs = Vec::with_capacity(args.len());
    let mut ptrs = Vec::with_capacity(args.len());

    for arg in args {
        let arg = arg.as_ref();
        let cstr = CString::new(arg).unwrap();
        let ptr = cstr.as_ptr();
        cstrs.push(cstr);
        ptrs.push(ptr);
    }

    unsafe {
        let ptr = func(ptrs.len() as i32, ptrs.as_slice().as_ptr());
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}
