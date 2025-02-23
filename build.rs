extern crate chrono;

fn main() {
    println!("cargo:rustc-env=VERSION={}", "2.0.0");
    println!("cargo:rustc-env=DATE={}", chrono::Local::now().format("%d/%m/%Y %H:%M"));
    if let Ok(result) = std::process::Command::new("git").args(["rev-parse", "HEAD"]).output() {
        println!("cargo:rustc-env=GIT_HASH={}", String::from_utf8(result.stdout).unwrap());
    }
}
