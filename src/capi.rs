use crate::OddCounter;
use static_assertions::const_assert_eq;
use std::ffi::OsString;
use subprocess::PopenConfig;
use subprocess::{Popen, PopenError};
use std::mem::MaybeUninit;
use std::mem;
use std::ffi::CStr;
use std::os::unix::ffi::OsStringExt;

// NB: The documentation comments from this file will be available
//     in the auto-generated header file example_project.h

/// Create new counter object given a start value.
///
/// On error (if an even start value is used), NULL is returned.
/// The returned object must be eventually discarded with example_project_oddcounter_free().
#[no_mangle]
pub extern "C" fn example_project_oddcounter_nnew(start: u32) -> Option<Box<OddCounter>> {
    OddCounter::new(start).ok().map(Box::new)
}

/// Discard a counter object.
///
/// Passing NULL is allowed.
#[no_mangle]
pub extern "C" fn example_project_oddcounter_free(_: Option<Box<OddCounter>>) {}

/// Increment a counter object.
#[no_mangle]
pub extern "C" fn example_project_oddcounter_increment(counter: &mut OddCounter) {
    counter.increment()
}

/// Obtain the current value of a counter object.
#[no_mangle]
pub extern "C" fn example_project_oddcounter_get_current(counter: &OddCounter) -> u32 {
    counter.current()
}

pub use crate::sizes::constants::String_SIZE;
#[repr(C)]
pub struct String_extern([u8; String_SIZE]);
const_assert_eq!(std::mem::size_of::<String>(), String_SIZE);
// Tried making a str struct:
//   pub struct Str_extern{[u8; std::mem::size_of::<str>()]);
// but got                                          ^^^ doesn't have a size known at compile-time
// Which makes sense, str is more of a trait-ish thing... but isn't it always
// a fat pointer that's 16ish bytes? Perhaps not.
// We'll go with String for now, for sending over the boundary.
// TODO: If we really want str, we could perhaps make a struct with usize + *const u8.

pub use crate::sizes::constants::OsString_SIZE;
#[repr(C)]
pub struct OsString_extern([u8; OsString_SIZE]);
const_assert_eq!(std::mem::size_of::<OsString>(), OsString_SIZE);

pub use crate::sizes::constants::Vec_Ref_OsString_SIZE;
#[repr(C)]
pub struct Vec_Ref_OsString_extern([u8; Vec_Ref_OsString_SIZE]);
const_assert_eq!(std::mem::size_of::<Vec<&OsString>>(), Vec_Ref_OsString_SIZE);
// Tried:
// #[repr(C)]
//   pub struct OsString_ref_slice_extern([u8; std::mem::size_of::<[&OsString_extern]>()]);
// But slices like [&OsString_extern] aren't known sized at compile time.
// So then tried:
//   #[repr(C)]
//   pub struct OsString_ref_slice_extern([u8; std::mem::size_of::<&[&OsString_extern]>()]);
// But then below when I tried to do:
//   let argv_ref: &[&OsString] = unsafe { &*(argv_raw as *const [&OsString]) };
// I got the error:                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   cannot cast thin pointer `*const OsString_ref_slice_extern` to fat pointer `*const [&OsString]`
// All these problems went away when using Vec, which was nice.
// TODO: If we want to send slices across, we might want to send around an integer and a pointer or something.

pub use crate::sizes::constants::PopenConfig_SIZE;
#[repr(C)]
pub struct PopenConfig_extern([u8; PopenConfig_SIZE]);
const_assert_eq!(std::mem::size_of::<PopenConfig>(), PopenConfig_SIZE);

pub use crate::sizes::constants::Popen_SIZE;
#[repr(C)]
pub struct Popen_extern([u8; Popen_SIZE]);
const_assert_eq!(std::mem::size_of::<Popen>(), Popen_SIZE);

pub use crate::sizes::constants::Result_Popen_PopenError_SIZE;
#[repr(C)]
pub struct Result_Popen_PopenError_extern([u8; Result_Popen_PopenError_SIZE]);
const_assert_eq!(std::mem::size_of::<Result<Popen, PopenError>>(), Result_Popen_PopenError_SIZE);

#[no_mangle]
pub extern "C" fn PopenConfig_default_extern(
    result_raw: *mut PopenConfig_extern,
) {
    let result_ptr: *mut PopenConfig =
        unsafe { mem::transmute(result_raw) };
    let result: PopenConfig =
        PopenConfig::default();
    unsafe { std::ptr::write(result_ptr, result) };
}

#[no_mangle]
pub extern "C" fn Vec_Ref_OsString_new(
    result_raw: *mut Vec_Ref_OsString_extern,
) {
    let result_ptr: *mut Vec<&OsString> =
        unsafe { mem::transmute(result_raw) };
    let result: Vec<&OsString> =
        Vec::new();
    unsafe { std::ptr::write(result_ptr, result) };
}

#[no_mangle]
pub extern "C" fn Vec_Ref_OsString_push(
    self_raw: *mut Vec_Ref_OsString_extern,
    element_raw: *const OsString_extern,
) {
    let self_ptr: *mut Vec<&OsString> =
        unsafe { mem::transmute(self_raw) };
    let self_ref: &mut Vec<&OsString> =
        unsafe { mem::transmute(self_ptr) };
    let element_ptr: *const OsString =
        unsafe { mem::transmute(element_raw) };
    let element_ref: &OsString =
        unsafe { mem::transmute(element_ptr) };
    self_ref.push(element_ref);
}

#[no_mangle]
pub extern "C" fn OsString_new_extern(
    cstr_raw: *const i8,
    cstr_len: usize,
    result_raw: *mut OsString_extern,
) {
  let result_ptr: *mut OsString =
      unsafe { mem::transmute(result_raw) };
  let cstr =
      unsafe { CStr::from_ptr(cstr_raw) };
  let bytes = cstr.to_bytes();
  let slice = &bytes[..cstr_len];
  let result =  OsString::from_vec(slice.to_vec());
  println!("{:?}", result);
  unsafe { std::ptr::write(result_ptr, result) };
}


#[no_mangle]
pub extern "C" fn String_into_OsString_extern(
    self_raw: *const String_extern,
    result_raw: *mut OsString_extern,
) {
    let self_ref: &OsString =
        unsafe { mem::transmute(self_raw) };
    let result_ptr: *mut OsString =
        unsafe { mem::transmute(result_raw) };
    let result = 
        self_ref.into();
    unsafe { std::ptr::write(result_ptr, result) };
}

#[no_mangle]
pub extern "C" fn Popen_create_extern(
    argv_raw: *const Vec_Ref_OsString_extern,
    config_raw: *mut PopenConfig_extern,
    result_raw: *mut Result_Popen_PopenError_extern,
) {
    let argv_ref: &Vec<&OsString> =
        unsafe { mem::transmute(argv_raw) };
    let config_ptr: *mut PopenConfig =
        unsafe { mem::transmute(config_raw) };
    let config: PopenConfig =
        unsafe { std::ptr::read(config_ptr) };
    let result_ptr: *mut Result<Popen, PopenError> =
        unsafe { mem::transmute(result_raw) };
    let result =
        Popen::create(argv_ref, config);
    unsafe { std::ptr::write(result_ptr, result) };
}

#[no_mangle]
pub extern "C" fn Result_Popen_PopenError_is_ok_extern(
    self_raw: *const Result_Popen_PopenError_extern
) -> u64 {
    let self_ptr: *const Result<Popen, PopenError> =
        unsafe { mem::transmute(self_raw) };
    let self_ref: &Result<Popen, PopenError> =
        unsafe { mem::transmute(self_ptr) };
    if self_ref.is_ok() {
      return 1;
    } else {
      return 0;
    }
}
