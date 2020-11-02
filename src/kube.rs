use std::process::Command;

pub fn get_secret(secret_name: &str) -> String {
    let cmd = Command::new("kubectl")
    .args(&["get", "secret", &secret_name, "-o", "json"])
    .output()
    .expect("Could not run kubectl");

  return String::from_utf8_lossy(&cmd.stdout).into_owned();
}