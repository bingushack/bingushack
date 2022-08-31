const HWID: &str = "";

use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    println!("cargo:rustc-env=HWID={}", HWID);
}