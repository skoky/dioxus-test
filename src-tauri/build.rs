use std::process::Command;

fn main() {
    // println!("cargo:rerun-if-env-changed=SHORT_SHA"); // allow CI override

    // // Prefer CI-provided value if present
    // let short = std::env::var("SHORT_SHA").ok().filter(|s| !s.is_empty()).unwrap_or_else(|| {
    //     let out = Command::new("git").args(["rev-parse", "--short=7", "HEAD"]).output().expect("failed to run git");
    //     String::from_utf8(out.stdout).unwrap().trim().to_string()
    // });
    //
    // println!("cargo:rustc-env=SHORT_SHA={}", short);

    tauri_build::build()
}
