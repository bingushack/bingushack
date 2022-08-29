const HWID: &str = "<hwid>";

fn main() {
    println!("cargo:rustc-env=HWID={}", HWID);
}