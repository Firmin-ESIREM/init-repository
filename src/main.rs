use std::process::{Command, Stdio, exit};
use chrono::{Utc, Datelike};
use std::fs::write;

fn main() {
    git_init_repository();
    write_license();
}

fn git_init_repository() {
    let cmd = Command::new("git").args(["init"]).stdin(Stdio::null()).stderr(Stdio::null()).stdout(Stdio::null()).status().expect("");
    if cmd.success() {
        println!("[v] Repository initialized.");
    } else {
        eprintln!("[x] Repository could not be initialized.");
        exit(1);
    }
}

fn write_license() {
    let year_string = Utc::now().year().to_string();
    let year: &str = &year_string;
    let license_template = "Copyright (c) {{ YEAR }} Firmin Launay (Étudiant à l’ESIREM)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
".replace("{{ YEAR }}", year);

    if let Err(_) = write("LICENSE", license_template) {
        eprintln!("[x] License file could not be written.");
        exit(1);
    }
    println!("[v] License file written.")
}
