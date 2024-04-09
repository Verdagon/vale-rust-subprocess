use std::process::Command;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ffi::OsString;
use subprocess::PopenConfig;
use subprocess::{Popen, PopenError};

fn main() {
    // Check if the build is targeting the same architecture and operating system as the host
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let host_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let host_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_arch == host_arch && target_os == host_os {
        println!("Building for the same architecture and OS as the host.");
        // Proceed with the build...
    } else {
        panic!("Build target does not match host architecture or OS.");
    }

    // It's probably not healthy to have build.rs generate something
    // inside the src directory like this...
    // I tried generating it in the output directory, so lib.rs could do:
    //   include!(concat!(env!("OUT_DIR"), "/constants.rs"));
    // but then cbindgen couldn't see it.
    // TODO: Maybe there's a way to have build.rs generate to somewhere that
    // cbindgen can see it, that's not in the src directory?
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&out_dir).join("src").join("sizes").join("constants.rs");
    let mut file = File::create(path).unwrap();

    writeln!(file, "pub const String_SIZE: usize = {};", std::mem::size_of::<String>()).unwrap();
    writeln!(file, "pub const OsString_SIZE: usize = {};", std::mem::size_of::<OsString>()).unwrap();
    writeln!(file, "pub const Vec_Ref_OsString_SIZE: usize = {};", std::mem::size_of::<Vec<&OsString>>()).unwrap();
    writeln!(file, "pub const PopenConfig_SIZE: usize = {};", std::mem::size_of::<PopenConfig>()).unwrap();
    writeln!(file, "pub const Popen_SIZE: usize = {};", std::mem::size_of::<Popen>()).unwrap();
    writeln!(file, "pub const Result_Popen_PopenError_SIZE: usize = {};", std::mem::size_of::<Result<Popen, PopenError>>()).unwrap();
}
