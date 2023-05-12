use std::io::stdin;
use std::path::Path;

use compute::compute;
use file::{
    delete_key_file, init, list_identifiers, read_decrypted_key_from_file,
    write_encrypted_key_to_file,
};

use crate::base32::decode;

mod base32;
mod compute;
mod file;

pub const TOTP_DIR_NAME: &str = ".totp";

const BIN_COMMAND: &str = "totp";
pub const COMMAND_HELP: &str = "--help";
pub const COMMAND_INIT: &str = "init";
pub const COMMAND_SHORT_INIT: &str = "i";
pub const COMMAND_COMPUTE: &str = "compute";
pub const COMMAND_SHORT_COMPUTE: &str = "c";
pub const COMMAND_LOAD: &str = "read";
pub const COMMAND_SHORT_LOAD: &str = "r";
pub const COMMAND_SAVE: &str = "save";
pub const COMMAND_SHORT_SAVE: &str = "s";
pub const COMMAND_DELETE: &str = "delete";
pub const COMMAND_LIST: &str = "list";
pub const COMMAND_SHORT_LIST: &str = "l";

const IDENTIFIER_LIST_HEADER: &str = "TOTP Store\n";
const IDENTIFIER_LIST_ITEM_PREFIX: &str = "├─";
const IDENTIFIER_LIST_LAST_ITEM_PREFIX: &str = "└─";

pub enum ErrorMessage<'a> {
    EmptyKey,
    MissingIdentifier(&'a str),
}

impl ErrorMessage<'_> {
    pub fn to_string(&self) -> String {
        match self {
            Self::EmptyKey => "Error: key must not be empty".to_string(),
            Self::MissingIdentifier(command) => {
                format!("Error: missing identifier - specify the identifier to {command}")
            }
        }
    }
}

impl Into<String> for ErrorMessage<'_> {
    fn into(self) -> String {
        String::from(self.to_string())
    }
}

pub fn get_help_text() -> String {
    format!(
        "TOTP Store - time-based one time password store

Usage:
    {BIN_COMMAND} [{COMMAND_INIT}, {COMMAND_SHORT_INIT}] <gpg-id>
        Initialize totp store with gpg-id for encrypting keys.

    {BIN_COMMAND} [{COMMAND_LIST}, {COMMAND_SHORT_LIST}]
        List all stored identifiers.

    {BIN_COMMAND} [{COMMAND_COMPUTE}, {COMMAND_SHORT_COMPUTE}] <identifier>
        Compute current one time password for given identifier.

    {BIN_COMMAND} {COMMAND_DELETE} <identifier>
        Delete identifier and key from store.

    {BIN_COMMAND} [{COMMAND_LOAD}, {COMMAND_SHORT_LOAD}] <identifier>
        Decrypt and output key of given identifier.

    {BIN_COMMAND} [{COMMAND_SAVE}, {COMMAND_SHORT_SAVE}] <identifier>
        Save key for given identifier.
        Prompts to overwrite existing files."
    )
}

pub fn run(gpg_home_dir: &Path, totp_dir: &Path, args: Vec<String>) -> Result<String, String> {
    let command = {
        if args.len() < 2 {
            COMMAND_LIST
        } else {
            args[1].as_str()
        }
    };

    match command {
        COMMAND_INIT | COMMAND_SHORT_INIT => {
            if args.len() < 3 {
                return Err(
                    "Error: gpg id required for initialization - totp init <gpg_id>".to_string(),
                );
            }
            let gpg_id = args[2].as_str();
            init(totp_dir, gpg_id)?;
            Ok(format!("totp store initialized with gpg id {}", gpg_id))
        }
        COMMAND_LIST | COMMAND_SHORT_LIST => Ok(print_list(&list_identifiers(totp_dir)?)),
        COMMAND_SAVE | COMMAND_SHORT_SAVE => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_SAVE).into());
            }
            let identifier = args[2].as_str();
            let key_base32 = read_key_input(identifier)?;
            write_encrypted_key_to_file(gpg_home_dir, totp_dir, identifier, &key_base32)?;
            Ok(format!("Key for {identifier} saved."))
        }
        COMMAND_LOAD | COMMAND_SHORT_LOAD => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_LOAD).into());
            }
            let identifier = args[2].as_str();
            match read_decrypted_key_from_file(gpg_home_dir, totp_dir, identifier)? {
                None => Ok(format!("Identifier {identifier} not found.")),
                Some(key) => Ok(format!("Key for identifier {identifier}: {key}")),
            }
        }
        COMMAND_DELETE => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_DELETE).into());
            }
            let identifier = args[2].as_str();
            delete_key_file(totp_dir, identifier)?;
            Ok(format!("Key for {identifier} deleted."))
        }
        COMMAND_COMPUTE | COMMAND_SHORT_COMPUTE => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_COMPUTE).into());
            }
            let identifier = args[2].as_str();
            let maybe_key_base32 = read_decrypted_key_from_file(gpg_home_dir, totp_dir, identifier)
                .map_err(|error| format!("Error reading file - {error}"))?;
            match maybe_key_base32 {
                None => {
                    return Err(format!("Error: no entry found for identifier {identifier}"));
                }
                Some(key_base32) => {
                    let key = decode(&key_base32)?;
                    let time = std::time::SystemTime::UNIX_EPOCH
                        .elapsed()
                        .map_err(|error| {
                            format!("Error: could not determine current system time - {error}",)
                        })?
                        .as_secs();
                    let time_step_interval = 30;
                    let time_step = time / time_step_interval;
                    let totp = compute(&key, time_step)?;
                    Ok(format!("Current TOTP for {identifier} is {totp}"))
                }
            }
        }
        COMMAND_HELP => Ok(get_help_text()),
        _ => Err(format!(
            "Error: unknown command \"{command}\"\n\n{}",
            get_help_text()
        )),
    }
}

fn read_key_input(identifier: &str) -> Result<String, String> {
    println!("Enter key for identifier {identifier}: ");
    let mut key_base32 = String::new();
    stdin()
        .read_line(&mut key_base32)
        .map_err(|error| format!("Error entering key: {}", error))?;
    if key_base32.is_empty() {
        return Err(ErrorMessage::EmptyKey.into());
    }
    key_base32 = key_base32
        .trim()
        .replace(" ", "")
        .to_string()
        .to_uppercase();
    // verify valid Base32 encoding of key
    base32::decode(&key_base32)?;
    Ok(key_base32)
}

fn print_list(identifier_list: &Vec<String>) -> String {
    let mut printed_list = String::from(IDENTIFIER_LIST_HEADER);
    if let Some((last_identifier, identifiers)) = identifier_list.split_last() {
        for identifier in identifiers {
            printed_list.push_str(format!("{IDENTIFIER_LIST_ITEM_PREFIX} {identifier}\n").as_str())
        }
        printed_list
            .push_str(format!("{IDENTIFIER_LIST_LAST_ITEM_PREFIX} {}", last_identifier).as_str());
    } else {
        printed_list.push_str("--- empty ---")
    }
    printed_list
}

#[cfg(test)]
mod tests {
    use crate::{
        print_list, IDENTIFIER_LIST_HEADER, IDENTIFIER_LIST_ITEM_PREFIX,
        IDENTIFIER_LIST_LAST_ITEM_PREFIX,
    };

    #[test]
    fn print_identifier_list() {
        let identifiers = vec![
            String::from("identifier_1"),
            String::from("identifier_2"),
            String::from("identifier_3"),
        ];
        let mut expected_printed_list = IDENTIFIER_LIST_HEADER.to_string();
        identifiers[0..identifiers.len() - 1]
            .iter()
            .for_each(|identifier| {
                expected_printed_list
                    .push_str(format!("{IDENTIFIER_LIST_ITEM_PREFIX} {identifier}\n").as_str())
            });
        expected_printed_list.push_str(
            format!(
                "{IDENTIFIER_LIST_LAST_ITEM_PREFIX} {}",
                identifiers.last().unwrap()
            )
            .as_str(),
        );

        let printed_list = print_list(&identifiers);

        assert_eq!(printed_list, expected_printed_list);
    }
}
