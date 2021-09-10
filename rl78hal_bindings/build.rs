use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, ffi::OsStr};
use std::process::Command;

fn main() {
    let rl78hal_top = "common";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // let out_dir = PathBuf::from("bin");

    let inc_dirs = [
        &format!("-I.", ),
        // &format!("-I{}/Inc", cube_top),
    ];
    let srcs = [
        [&format!("{}/", rl78hal_top), "init.c"],
        [&format!("{}/", rl78hal_top), "vect.c"],
        [&format!("{}/", rl78hal_top), "option_bytes.c"],
        [&format!("{}/", rl78hal_top), "rust_main.c"],
    ];
    
    let defines = [
        "-DSIG_G13"
    ];

    let mut objs: Vec<String> = Vec::new();

    Command::new("rl78-elf-gcc")
        .arg("-c")
        .args(&["-mcpu=g13"])
        .args(&defines)
        .args(&inc_dirs)
        .args(&["common/start.s"])
        .args(&["-o"])
        .arg(&format!("{}/start.o", out_dir.display()))
        .status().unwrap();
    
    for src in &srcs {
        let obj = src[1].to_string().replace(".c", ".o");
    
        Command::new("rl78-elf-gcc")
            .arg("-c")
            .args(&["-mcpu=g13"])
            .args(&defines)
            .args(&inc_dirs)
            .arg(&format!("{}/{}", src[0], src[1]))
            .arg("-o")
            .arg(&format!("{}/{}", out_dir.display(), obj))
            .status().unwrap();
    
        objs.push(obj);
    }

    Command::new("rl78-elf-ar")
         .args(&["crs", "librl78.a"])
         .arg(&format!("{}/start.o", out_dir.display()))
         .args(&objs)
         .current_dir(&Path::new(&out_dir))
         .status().unwrap();

    // Command::new("rl78-elf-ar")
    //      .args(&["crs", "rl78.a"])
    //      .arg(&format!("{}/start.o", out_dir.display()))
    //      .args(&objs)
    //      .current_dir(&Path::new(&out_dir))
    //      .status().unwrap();

    let _target = "rl78";
    let _name = "rl78";

        fs::copy(
            //format!("{}/{}.a", out_dir.display(), _target),
            //out_dir.join(format!("lib{}.a", _name)),
            format!("bin/lib{}.a", _target),
            //format!("bin/{}.a", _target),
            out_dir.join("librl78.a"),
        )
        .unwrap();

        // println!("cargo:rustc-link-lib=static=rl78");
        // println!("cargo:rustc-link-lib=static={}", _name);
        println!("cargo:rustc-link-lib={}", _name);
        println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed={}/{}.a", out_dir.display(), _target);
    println!("cargo:rerun-if-changed=build.rs");
}
