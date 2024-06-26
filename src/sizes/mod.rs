
// constants.rs is generated by the build.rs build step
pub mod constants;

// Example generated constants.rs:
//
// pub const String_SIZE: usize = 24;
// pub const OsString_SIZE: usize = 24;
// pub const Vec_Ref_OsString_SIZE: usize = 24;
// pub const PopenConfig_SIZE: usize = 144;
// pub const Popen_SIZE: usize = 24;
// pub const Result_Popen_PopenError_SIZE: usize = 24;

// It's probably not healthy to have build.rs generate something
// inside the src directory like this...
// I tried generating it in the output directory, so lib.rs could do:
//   include!(concat!(env!("OUT_DIR"), "/constants.rs"));
// but then cbindgen couldn't see it.
// TODO: Maybe there's a way to have build.rs generate to somewhere that
// cbindgen can see it, that's not in the src directory?
