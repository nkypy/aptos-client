fn main() {
    println!("cargo:rustc-env=CC=emcc");
    println!("cargo:rustc-env=AR=emar");
    // let target = env::var("TARGET").unwrap();
    // if target.contains("wasm32") {
    //     println!("set wasm32 env CC to emcc and AR to emar");
    //     println!("cargo:rustc-env=CC=emcc");
    //     println!("cargo:rustc-env=AR=emar");
    // }
}
