use std::path::PathBuf;

fn main() {
    let clipper2_root = PathBuf::from("Clipper2/CPP/Clipper2Lib");
    let include_dir = clipper2_root.join("include");
    let src_dir = clipper2_root.join("src");

    // Verify source exists (catches missing submodule)
    if !include_dir.exists() {
        panic!(
            "Clipper2 source not found at {:?}. \
             Run: git submodule update --init",
            include_dir
        );
    }

    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .include(&include_dir)
        .file(src_dir.join("clipper.engine.cpp"))
        .file(src_dir.join("clipper.offset.cpp"))
        .file(src_dir.join("clipper.rectclip.cpp"))
        .compile("clipper2");

    // Export paths for downstream crates via DEP_CLIPPER2_* variables
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!(
        "cargo:include={}",
        include_dir.canonicalize().unwrap().display()
    );
    println!(
        "cargo:root={}",
        clipper2_root.canonicalize().unwrap().display()
    );
    println!("cargo:lib={out_dir}");

    println!("cargo:rerun-if-changed=Clipper2/CPP/Clipper2Lib");
}
