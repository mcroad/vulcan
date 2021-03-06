use std::process::Command;

fn main() {
  println!("cargo:rerun-if-changed=../.git/HEAD");

  let output = Command::new("git")
    .args(&["rev-parse", "--short", "HEAD"])
    .output()
    .unwrap();
  let git_short_hash = String::from_utf8(output.stdout).unwrap();
  println!("cargo:rustc-env=GIT_HASH={}", git_short_hash);
}
