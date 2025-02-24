extern crate chrono;

fn main() {
    println!("cargo:rustc-env=VERSION={}", "2.0.0");
    println!("cargo:rustc-env=DATE={}", chrono::Local::now().format("%d/%m/%Y %H:%M:%S %z"));
    println!("cargo:rustc-env=GIT_HASH={}", if let Ok(result) = std::process::Command::new("git").args(["rev-parse", "HEAD"]).output() {
        String::from_utf8(result.stdout).unwrap()
    } else {
        "Could not run `git rev-parse HEAD` at compile time.".to_string()
    });
}
