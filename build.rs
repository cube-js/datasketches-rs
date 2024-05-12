use std::path::PathBuf;

fn main() {
    let datasketches = PathBuf::from("datasketches-cpp");
    let src = PathBuf::from("src");
    let mut bridge = cxx_build::bridge(src.join("bridge.rs"));

    bridge
        .files(&[
            datasketches.join("cpc.cpp"),
            datasketches.join("hll.cpp"),
            datasketches.join("theta.cpp"),
            datasketches.join("hh.cpp"),
        ])
        .include(datasketches.join("common").join("include"))
        .cpp_link_stdlib(None)
        .static_flag(true);

    // MSVC doesn't plan to implement C++11 switch, because they use c++14 by default
    #[cfg(not(target_env = "msvc"))]
    bridge.flag_if_supported("-std=c++11");

    bridge.compile("libdatasketches.a");
}
