use std::{env, path::Path, process};

use totp::run;

const FILE_PATH: &str = "keys.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(FILE_PATH);
    match run(args, file_path) {
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1)
        }
        Ok(output) => println!("{}", output),
    }
}
