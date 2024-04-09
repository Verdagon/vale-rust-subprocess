use static_assertions::const_assert_eq;
use std::ffi::OsString;
use subprocess::PopenConfig;
use subprocess::{Popen, PopenError};
use std::mem::MaybeUninit;
use std::mem;
// mod sizes;
// use sizes::String_extern_SIZE;
// pub const String_extern_SIZE: usize = 24;

// include!(concat!(env!("OUT_DIR"), "/sizes.rs"));

pub mod sizes;

// Tried:
//   pub struct Str_extern{[u8; std::mem::size_of::<str>()]);
// but got                                          ^^^ doesn't have a size known at compile-time
// So we'll go with String for now.
// If we really want str, we might try usize + *const u8, though C/Vale will
// need to make sure the chars remain there while the pointer is alive.
pub use crate::sizes::constants::String_SIZE;
#[repr(C)]
pub struct String_extern([u8; String_SIZE]);
const_assert_eq!(std::mem::size_of::<String>(), String_SIZE);

pub use crate::sizes::constants::OsString_SIZE;
#[repr(C)]
pub struct OsString_extern([u8; OsString_SIZE]);

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
// So settled on the above with the integer.

#[repr(C)]
pub struct Vec_Ref_OsString_extern([u8; std::mem::size_of::<Vec<&OsString>>()]);

#[repr(C)]
pub struct PopenConfig_extern([u8; std::mem::size_of::<PopenConfig>()]);

#[repr(C)]
pub struct Popen_extern([u8; std::mem::size_of::<Popen>()]);

#[repr(C)]
pub struct Result_Popen_PopenError_extern([u8; std::mem::size_of::<Result<Popen, PopenError>>()]);


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // We start with these String instances.
        // This isn't quite realistic, we'll still need to create
        // these from the C side (TODO).
        let program_name_string = String::from("/bin/cat");
        let arg1_string = String::from("/Users/verdagon/hello.txt");

        // Create program_name OsString from String
        //   let program_name_osstring: OsString = arg1.into();
        let mut program_name_osstring_uninit: MaybeUninit<OsString> =
            unsafe { MaybeUninit::uninit() };
        String_into_OsString_extern(
            unsafe { mem::transmute(&program_name_string) },
            program_name_osstring_uninit.as_mut_ptr() as *mut OsString_extern);
        let program_name_osstring: OsString =
            unsafe { program_name_osstring_uninit.assume_init() };


        // Create arg1 OsString from String
        //   let arg1_osstring: OsString = arg1.into();
        let mut arg1_osstring_uninit: MaybeUninit<OsString> =
            unsafe { MaybeUninit::uninit() };
        String_into_OsString_extern(
            unsafe { mem::transmute(&arg1_string) },
            arg1_osstring_uninit.as_mut_ptr() as *mut OsString_extern);
        let arg1_osstring: OsString =
            unsafe { arg1_osstring_uninit.assume_init() };

        // Create vec
        //   let argv: Vec<&OsString> = Vec::new();
        let mut argv_uninit: MaybeUninit<Vec<&OsString>> =
            unsafe { MaybeUninit::uninit() };
        Vec_Ref_OsString_new(
            argv_uninit.as_mut_ptr() as *mut Vec_Ref_OsString_extern);
        let mut argv: Vec<&OsString> =
            unsafe { argv_uninit.assume_init() };

        // Push first element into argv vec
        //   argv.push(&program_name_osstring)
        Vec_Ref_OsString_push(
          unsafe { mem::transmute(&mut argv) },
          unsafe { mem::transmute(&program_name_osstring) });

        // Push second element into argv vec
        //   argv.push(&arg1_osstring)
        Vec_Ref_OsString_push(
          unsafe { mem::transmute(&mut argv) },
          unsafe { mem::transmute(&arg1_osstring) });

        // Make default PopenConfig
        //   let popen_config = PopenConfig::default();
        let mut popen_config_uninit: MaybeUninit<PopenConfig> =
            unsafe { MaybeUninit::uninit() };
        PopenConfig_default_extern(
            popen_config_uninit.as_mut_ptr() as *mut PopenConfig_extern);
        let mut popen_config: PopenConfig =
            unsafe { popen_config_uninit.assume_init() };

        // Call Popen::create
        //   let result = Popen::create(argv, popen_config);
        let argv_raw: *const Vec_Ref_OsString_extern =
            unsafe { mem::transmute(&argv) };
        let config_raw: *mut PopenConfig_extern =
            unsafe { mem::transmute(&mut popen_config) };
        let mut result_uninit: MaybeUninit<Result<Popen, PopenError>> =
            unsafe { MaybeUninit::uninit() };
        let result_raw: *mut Result_Popen_PopenError_extern =
            result_uninit.as_mut_ptr() as *mut Result_Popen_PopenError_extern;
        Popen_create_extern(argv_raw, config_raw, result_raw);
        // popen_config has been moved, can't use it anymore

        // Make sure it's okay. Not quite realistic as we need to call
        // is_ok from the C side (TODO).
        let result = unsafe { result_uninit.assume_init() };
        assert_eq!(result.is_ok(), true);
    }
}

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

