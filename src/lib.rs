use std::io::stdin;
use std::path::Path;

use compute::compute;
use file::{delete_key_from_file, list_identifiers, read_key_from_file};

use crate::base32::decode;
use crate::file::{
    ensure_file_exists, identifier_exists_in_file, update_key_in_file, write_key_to_file,
};

mod base32;
pub mod compute;
mod file;

pub const COMMAND_COMPUTE: &str = "compute";
pub const COMMAND_SHORT_COMPUTE: &str = "c";
pub const COMMAND_LOAD: &str = "read";
pub const COMMAND_SHORT_LOAD: &str = "r";
pub const COMMAND_SAVE: &str = "save";
pub const COMMAND_SHORT_SAVE: &str = "s";
pub const COMMAND_UPDATE: &str = "update";
pub const COMMAND_SHORT_UPDATE: &str = "u";
pub const COMMAND_DELETE: &str = "delete";
pub const COMMAND_SHORT_DELETE: &str = "d";
pub const COMMAND_LIST: &str = "list";
pub const COMMAND_SHORT_LIST: &str = "l";

const IDENTIFIER_LIST_HEADER: &str = "TOTP identifiers\n";
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
                format!("Error: missing identifier - specify the identifier to use with {command}")
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
        "Usage: totp <command> [<identifier>]

Commands:
    {COMMAND_COMPUTE}, {COMMAND_SHORT_COMPUTE}   compute current password for given identifier
    {COMMAND_DELETE}, {COMMAND_SHORT_DELETE}    delete entry of given identifier
    {COMMAND_LOAD}, {COMMAND_SHORT_LOAD}      output associated key of given identifier
    {COMMAND_SAVE}, {COMMAND_SHORT_SAVE}      save key for given identifier
    {COMMAND_UPDATE}, {COMMAND_SHORT_UPDATE}    prompt to update key of given identifier"
    )
}

pub fn run(args: Vec<String>, file_path: &Path) -> Result<String, String> {
    let command = {
        if args.len() < 2 {
            COMMAND_LIST
        } else {
            args[1].as_str()
        }
    };

    match command {
        COMMAND_SAVE | COMMAND_SHORT_SAVE => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_SAVE).into());
            }
            ensure_file_exists(file_path)?;

            let identifier = args[2].as_str();
            let identifier_exists = identifier_exists_in_file(identifier, file_path)?;
            if identifier_exists {
                return Err(format!("Error: identifier {identifier} already exists"));
            }

            let key_base32 = read_key_for_identifier(identifier)?;
            write_key_to_file(&identifier.to_string(), &key_base32, file_path)?;
            Ok(format!("Key for identifier {identifier} saved."))
        }
        COMMAND_LOAD | COMMAND_SHORT_LOAD => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_LOAD).into());
            }
            ensure_file_exists(file_path)?;

            let identifier = args[2].as_str();
            match read_key_from_file(identifier, file_path)? {
                None => Ok(format!("Identifier not found.")),
                Some(key) => Ok(format!("Key for identifier {identifier}: {key}")),
            }
        }
        COMMAND_UPDATE | COMMAND_SHORT_UPDATE => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_UPDATE).into());
            }
            let identifier = args[2].as_str();
            let identifier_exists = identifier_exists_in_file(identifier, file_path)?;
            if !identifier_exists {
                return Err(format!("Error: identifier {identifier} does not exist"));
            }

            let key_base32 = read_key_for_identifier(identifier)?;
            update_key_in_file(identifier, &key_base32, file_path)?;
            Ok(format!("Entry for identifier {identifier} updated."))
        }
        COMMAND_DELETE | COMMAND_SHORT_DELETE => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_DELETE).into());
            }
            let identifier = args[2].as_str();
            delete_key_from_file(identifier, file_path)?;
            Ok(format!("Entry for identifier {identifier} deleted."))
        }
        COMMAND_LIST | COMMAND_SHORT_LIST => Ok(print_list(&list_identifiers(file_path)?)),
        COMMAND_COMPUTE | COMMAND_SHORT_COMPUTE => {
            if args.len() < 3 {
                return Err(ErrorMessage::MissingIdentifier(COMMAND_COMPUTE).into());
            }
            let identifier = args[2].as_str();
            let maybe_key_base32 = read_key_from_file(identifier, file_path)
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
        _ => Err(format!(
            "Error: unknown command \"{command}\"\n\n{}",
            get_help_text()
        )),
    }
}

fn read_key_for_identifier(identifier: &str) -> Result<String, String> {
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
    // test key for valid Base32 encoding
    base32::decode(&key_base32)?;
    Ok(key_base32)
}

fn print_list(identifier_list: &Vec<String>) -> String {
    let mut printed_list = String::from(IDENTIFIER_LIST_HEADER);
    identifier_list[0..identifier_list.len() - 1]
        .iter()
        .for_each(|identifier| {
            printed_list.push_str(format!("{IDENTIFIER_LIST_ITEM_PREFIX} {identifier}\n").as_str())
        });
    printed_list.push_str(
        format!(
            "{IDENTIFIER_LIST_LAST_ITEM_PREFIX} {}",
            identifier_list[identifier_list.len() - 1]
        )
        .as_str(),
    );
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
