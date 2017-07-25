fn main() {
    add_libs();
}

#[cfg(windows)]
fn add_libs() {
    println!("cargo:rustc-link-lib=user32");
}

#[cfg(not(windows))]
fn add_libs() {}
