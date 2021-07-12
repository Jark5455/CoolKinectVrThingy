fn main() {
    println!("cargo:rustc-link-lib=dylib=NiTE2");
    println!("cargo:rustc-link-lib=dylib=OpenNI2");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}
