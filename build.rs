// Command is used to execute external commands i.e. Git
use std::process::Command;

/// This build script sets up three environment variables (GIT_HASH,
/// GIT_MESSAGE, and BUILD_DATE) that can be accessed in your Rust code using
/// the env! macro. It's a common pattern for embedding version information
/// into a compiled binary.
fn main() {
    // Get the Git hash
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output();
    // check if command executed successfully
    // - if yes, `output` contains command's output
    if let Ok(output) = output {
        // try to convert vector of bytes to UTF-8 string
        // - if successful, `git_hash` contains that string
        if let Ok(git_hash) = String::from_utf8(output.stdout) {
            // set Cargo env variable named GIT_HASH
            // - GIT_HASH variable will be available during compilation
            // - so main.rs can access using the env!() macro
            println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
        }
    }
    // Get the Git commit message
    if let Ok(output) = Command::new("git")
        .args(["log", "-1", "--pretty=%B"])
        .output()
    {
        if let Ok(git_message) = String::from_utf8(output.stdout) {
            // Escape any quotation marks in the message
            let escaped_message = git_message.replace("\"", "\\\"").trim().to_string();
            println!("cargo:rustc-env=GIT_MESSAGE={}", escaped_message);
        }
    }
    // Get the build date
    let build_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
}
