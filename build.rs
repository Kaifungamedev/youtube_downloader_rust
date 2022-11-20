#[cfg(target_os = "windows")]
use winres;

#[cfg(target_os = "windows")]
fn main() {
    use std::io::Write;
    if std::env::consts::OS == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./icon.ico");
        match res.compile() {
            Err(error) => {
                write!(std::io::stderr(), "{}", error).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
