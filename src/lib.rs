use std::ffi::OsString;
use subprocess::PopenConfig;
use subprocess::{Popen, PopenError};
use std::mem::MaybeUninit;
use std::mem;

// had to use OsString instead of OsStr

#[repr(C)]
pub struct OsString_extern([u8; std::mem::size_of::<OsString>()]);

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
pub struct Vec_Ref_OsString_extern([u8; std::mem::size_of::<Vec<OsString>>()]);

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
        let program_name = OsString::from("/bin/cat");
        let arg1 = OsString::from("/Users/verdagon/hello.txt");
        let argv: Vec<&OsString> = vec![&program_name, &arg1];
        let mut popen_config = PopenConfig::default();

        // Should wrap it first maybe?
        let argv_raw: *const Vec_Ref_OsString_extern =
            unsafe { mem::transmute(&argv) };
        let config_raw: *mut PopenConfig_extern =
            unsafe { mem::transmute(&mut popen_config) };
        let mut result_uninit: MaybeUninit<Result<Popen, PopenError>> =
            unsafe { MaybeUninit::uninit() };
        let result_raw: *mut Result_Popen_PopenError_extern =
            result_uninit.as_mut_ptr() as *mut Result_Popen_PopenError_extern;
        create_extern(argv_raw, config_raw, result_raw);
        // popen_config has been moved, can't use it anymore

        let result = unsafe { result_uninit.assume_init() };
        assert_eq!(result.is_ok(), true);
    }
}


#[no_mangle]
pub extern "C" fn create_extern(
    argv_raw: *const Vec_Ref_OsString_extern,
    config_raw: *mut PopenConfig_extern,
    result_raw: *mut Result_Popen_PopenError_extern,
) {
    let argv_ref: &Vec<&OsString> = unsafe { mem::transmute(argv_raw) };
    // let config_ref: &mut PopenConfig = unsafe { mem::transmute(config_raw) };
    let config_ptr: *mut PopenConfig = unsafe { mem::transmute(config_raw) };
    let config: PopenConfig = unsafe { std::ptr::read(config_ptr) };
    let result_ref: &mut Result<Popen, PopenError> = unsafe { &mut*(result_raw as *mut Result<Popen, PopenError>) };
    *result_ref = Popen::create(argv_ref, config);
}

