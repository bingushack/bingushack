// sopa's
// const HWID: &str = "dee2ce704deb97081914ecca6d1555ba6bb5658a493afe65427f64b00fa2bf04dbc53e5baf1cc38a4e94203f45bc82a0e6acbd1abf11afa6a66b7ef7e13acc3b";
const HWID: &str = "dee2ce704deb97081914ecca6d1555ba6bb5658a493afe65427f64b00fa2bf04dbc53e5baf1cc38a4e94203f45bc82a0e6acbd1abf11afa6a66b7ef7e13acc3b";

use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    println!("cargo:rustc-env=HWID={}", HWID);
}
