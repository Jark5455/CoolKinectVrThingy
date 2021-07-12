fn main() {
    println!("cargo:rustc-link-lib=dylib=NiTE2");
    println!("cargo:rustc-link-lib=dylib=OpenNI2");
    println!("cargo:rustc-link-lib=dylib=FreenectDriver");
    println!("cargo:rustc-link-search=native=/home/yashr/Documents/CoolKinectVrThingy/libs/NiTE-Linux-x64-2.2/Samples/Bin/");
    println!("cargo:rustc-link-search=native=/home/yashr/Documents/CoolKinectVrThingy/libs/NiTE-Linux-x64-2.2/Samples/Bin/OpenNI2/Drivers/");
}
