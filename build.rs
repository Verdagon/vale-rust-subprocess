use cargo_metadata::*;
use std::path::*;
use std::process::Command;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ffi::OsString;
use subprocess::PopenConfig;
use subprocess::{Popen, PopenError, Result};

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
    let constants_path = Path::new(&out_dir).join("src").join("sizes").join("constants.rs");
    let mut constants_file = File::create(constants_path).unwrap();

    writeln!(constants_file, "pub const String_SIZE: usize = {};", std::mem::size_of::<String>()).unwrap();
    writeln!(constants_file, "pub const OsString_SIZE: usize = {};", std::mem::size_of::<OsString>()).unwrap();
    writeln!(constants_file, "pub const Vec_Ref_OsString_SIZE: usize = {};", std::mem::size_of::<Vec<&OsString>>()).unwrap();
    writeln!(constants_file, "pub const PopenConfig_SIZE: usize = {};", std::mem::size_of::<PopenConfig>()).unwrap();
    writeln!(constants_file, "pub const Popen_SIZE: usize = {};", std::mem::size_of::<Popen>()).unwrap();
    writeln!(constants_file, "pub const Result_Popen_PopenError_SIZE: usize = {};", std::mem::size_of::<Result<Popen>>()).unwrap();

    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let meta = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(&path)
        .exec()
        .unwrap();

    println!("{:?}", meta);

    let out = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out);

    let path = out.join("capi/include/");
    let subdir = path.join("subdir");
    let include = out.join("include");

    std::fs::create_dir_all(&path).unwrap();
    std::fs::create_dir_all(&subdir).unwrap();
    std::fs::create_dir_all(&include).unwrap();

    std::fs::write(path.join("generated.h"), "// Generated").unwrap();
    std::fs::write(subdir.join("in_subdir.h"), "// Generated").unwrap();
    std::fs::write(include.join("other_file.h"), "// Generated").unwrap();
}
