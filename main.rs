use std::process::{Command, Stdio, exit};

fn main() {
    let cmd = Command::new("git").args(["init"]).stdin(Stdio::null()).stderr(Stdio::null()).stdout(Stdio::null()).status().expect("");
    if cmd.success() {
        println!("[v] Repository initialized.");
    } else {
        eprintln!("[x] Repository could not be initialized.");
        exit(1);
    }

}
