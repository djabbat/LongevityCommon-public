// build.rs — inject GIT_SHA + BUILD_TS at compile time for /api/version.
// Phase 4.6 ops hardening (2026-05-08).

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=GIT_SHA");
    println!("cargo:rerun-if-env-changed=BUILD_TS");

    let git_sha = std::env::var("GIT_SHA").ok().or_else(|| {
        Command::new("git")
            .args(["rev-parse", "--short=12", "HEAD"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    });
    if let Some(sha) = git_sha {
        println!("cargo:rustc-env=GIT_SHA={sha}");
    }

    let ts = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    println!("cargo:rustc-env=BUILD_TS={ts}");
}
