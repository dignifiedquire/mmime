extern crate cc;

fn link_dylib(lib: &str) {
    println!("cargo:rustc-link-lib=dylib={}", lib);
}

fn link_static(lib: &str) {
    println!("cargo:rustc-link-lib=static={}", lib);
}

fn add_search_path(p: &str) {
    println!("cargo:rustc-link-search={}", p);
}

fn main() {
    add_search_path("/usr/local/lib");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("-apple") || target.contains("-darwin") {
        link_dylib("iconv");
    } else if target.contains("-android") {
        link_static("iconv");
    } else if target.contains("-linux") {
    } else {
        panic!("unsupported target");
    }
}
