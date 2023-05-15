use std::{env, path::Path, process};

use totp_store::{run, TOTP_DIR_NAME};

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_dir = std::env::var("HOME").unwrap_or_else(|_| "./".to_string());
    let base_dir_path = Path::new(&base_dir);
    let gpg_home_dir = base_dir_path.join(".gnupg");
    let totp_dir = base_dir_path.join(TOTP_DIR_NAME);
    match run(&gpg_home_dir, &totp_dir, args) {
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1)
        }
        Ok(output) => println!("{}", output),
    }
}
