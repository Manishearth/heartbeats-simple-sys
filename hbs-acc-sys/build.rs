extern crate pkg_config;

use std::env;
use std::fs::{self, create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let pc = pkg_config::find_library("hbs-acc-static");
    if pc.is_err() {
        let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                               .join("heartbeats-simple");
        let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap());
        let _ = fs::create_dir(&dst);
        let build = src.join("_build");
        let target: String = env::var("TARGET").unwrap();
        let target_parts: Vec<&str> = target.split('-').collect();
        let cmake_var = match target_parts[target_parts.len() - 1].starts_with("android") {
            true => format!("-DCMAKE_TOOLCHAIN_FILE={}",
                            src.join("cmake-toolchain").join("android.toolchain.cmake").display()),
            false => "".to_owned(),
        };
        remove_dir_all(&build).ok();
        create_dir_all(&build).unwrap();
        run(Command::new("cmake").arg(cmake_var).arg(src.to_str().unwrap()).current_dir(&build));
        run(Command::new("make").arg("hbs-acc-static").current_dir(&build));
        println!("cargo:rustc-link-lib=static=hbs-acc-static");
        println!("cargo:rustc-link-search=native={}", build.join("lib").display());
    }
}

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}
