// sopa's old
// const HWID: &str = "dee2ce704deb97081914ecca6d1555ba6bb5658a493afe65427f64b00fa2bf04dbc53e5baf1cc38a4e94203f45bc82a0e6acbd1abf11afa6a66b7ef7e13acc3b";

// sopa's new
const HWID: &str = "8f314714555939bb057b5157ccb8338124276d2870660a4c4d4d1138dfabcc2c9eac246f615a6d1ee3a06fd4ca48f8978af293ef94d64118b1f2e385e5bc146f";

// MR_HOUS3's
// const HWID: &str = "88b69e6bf67b625b04b0b2c3a62a41a627588c2d13592a1ae53657daa0865ab19ef0511b14f8790ef76806e68199284893cf7af236f20883d068f3a52dd377e7";

// gite's
// const HWID: &str = "d2cb257cc97b402779cc428798ffe29fb4863963ad37ce257d242f492bb804630b0a898601e9864c1723a7a0a51dc83570d1cf08fb758522b5c993c2fcb0a638";

use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    println!("cargo:rustc-env=HWID={}", HWID);
}
