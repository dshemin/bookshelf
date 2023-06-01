use std::process::Command;

fn main() {
    println!("cargo:rustc-env=GIT_HASH={}", git_hash().unwrap());
}

fn git_hash() -> anyhow::Result<String> {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output()?;
    let git_hash = String::from_utf8(output.stdout)?;

    Ok(git_hash)
}
