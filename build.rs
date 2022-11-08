use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("wasm32") {
        println!("set wasm32 env CC to emcc and AR to emar");
        env::set_var("CC", "emcc");
        env::set_var("AR", "emar");
    }
}
