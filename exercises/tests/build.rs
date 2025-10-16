//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // The timestamp ensures the build script is re-run every time,
    // because its output changes. This forces the environment variable
    // to be updated on each build.
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // This tells the compiler to act as if `--cfg pass` was passed to it.
    // This will enable any code annotated with `#[cfg(pass)]`.
    let your_command = "rustc-cfg=feature = \"pass\"";
    println!("cargo:{}", your_command);
}