extern crate gcc;

use std::{env, fs};
use std::path::{Path, PathBuf};

fn main() {
    let udis_dir = Path::new("libudis86");
    let files = [
        "decode.c",
        "itab.c",
        "syn-att.c",
        "syn-intel.c",
        "syn.c",
        "udis86.c"
    ];

    let mut config = gcc::Config::new();
    for file in files.iter() {
        config.file(udis_dir.join(file));
    }

    config.include(udis_dir)
          .flag("-includestring.h")
          .compile("libudis86.a");
    println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
