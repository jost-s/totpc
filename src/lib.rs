use std::io::stdin;
use std::path::Path;

use compute::compute;
use file::{delete_key_from_file, read_key_from_file};

use crate::base32::decode;
use crate::file::{ensure_file_exists, identifier_exists_in_file, write_key_to_file};

mod base32;
pub mod compute;
mod file;

pub const COMMAND_COMPUTE: &str = "compute";
pub const COMMAND_LOAD: &str = "read";
pub const COMMAND_SAVE: &str = "save";
pub const COMMAND_DELETE: &str = "delete";

pub enum ErrorMessage {
    EmptyKey,
    MissingIdentifier,
}

impl ErrorMessage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyKey => "Error: key must not be empty",
            Self::MissingIdentifier => {
                "Error: missing identifier - specify the identifier to use for the command"
            }
        }
    }
}

pub fn print_help() {
    println!("The One The Password");
    println!("Usage: totp [command] <identifier>");
    println!();
    println!(
        "All possible commands are:\n- {}\n- {}\n- {}\n- {}",
        COMMAND_COMPUTE, COMMAND_LOAD, COMMAND_SAVE, COMMAND_DELETE
    );
}

pub fn run(args: Vec<String>, file_path: &Path) -> Result<String, String> {
    if args.len() < 2 {
        print_help();
        println!();
        return Err(format!("No command specified."));
    }
    let command = args[1].as_str();

    match command {
        COMMAND_SAVE => {
            if args.len() < 3 {
                return Err(format!("{}", ErrorMessage::MissingIdentifier.as_str()));
            }
            ensure_file_exists(file_path)
                .map_err(|error| format!("Error creating file - {}", error))?;

            let identifier = args[2].as_str();
            let identifier_exists = identifier_exists_in_file(identifier, file_path)
                .map_err(|error| format!("Error reading file - {}", error))?;
            if identifier_exists {
                return Err(format!("Error: identifier {} already exists", identifier));
            }

            println!("Enter key for identifier {}: ", identifier);
            let mut key_base32 = String::new();
            stdin()
                .read_line(&mut key_base32)
                .map_err(|error| format!("Error entering key: {}", error))?;
            key_base32 = key_base32.trim().replace(" ", "").to_string();
            if key_base32.is_empty() {
                return Err(format!("{}", ErrorMessage::EmptyKey.as_str()));
            }
            // test key for valid Base32 encoding
            base32::decode(&key_base32)?;

            write_key_to_file(file_path, &identifier.to_string(), &key_base32)
                .map_err(|error| format!("Error: could not create file to save key - {}", error))?;
            Ok(format!("Key for identifier {} saved.", identifier))
        }
        COMMAND_COMPUTE => {
            if args.len() < 3 {
                return Err(format!("{}", ErrorMessage::MissingIdentifier.as_str()));
            }
            let identifier = args[2].as_str();
            let maybe_key_base32 = read_key_from_file(file_path, identifier)
                .map_err(|error| format!("Error reading file - {}", error))?;
            match maybe_key_base32 {
                None => {
                    return Err(format!(
                        "Error: no entry found for identifier {}",
                        identifier
                    ));
                }
                Some(key_base32) => {
                    let key = decode(&key_base32)?;
                    let time = std::time::SystemTime::UNIX_EPOCH
                        .elapsed()
                        .map_err(|error| {
                            format!("Error: could not determine current system time - {}", error)
                        })?
                        .as_secs();
                    let time_step_interval = 30;
                    let time_step = time / time_step_interval;
                    let totp = compute(&key, time_step)?;
                    Ok(format!("Current TOTP for {} is {}", identifier, totp))
                }
            }
        }
        COMMAND_DELETE => {
            if args.len() < 3 {
                return Err(format!("{}", ErrorMessage::MissingIdentifier.as_str()));
            }
            let identifier = args[2].as_str();
            delete_key_from_file(identifier, file_path)?;
            Ok(format!("Entry for identifier {} deleted.", identifier))
        }
        _ => Err(format!("Error: unknown command \"{}\"", command)),
    }
}
