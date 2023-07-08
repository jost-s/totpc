use std::{
    ffi::OsStr,
    fs::{create_dir, read_to_string, remove_file, write},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

const GPG_COMMAND: &str = "gpg";
const GPG_ID_FILE_NAME: &str = ".gpg-id";
const GPG_FILE_EXTENSION: &str = "gpg";

/// Initialize a directory for usage with totpc. Creates a file with the GPG id
/// in it.
pub fn init(totp_dir: &Path, gpg_id: &str) -> Result<(), String> {
    let gpg_id_file = totp_dir.join(GPG_ID_FILE_NAME);
    if gpg_id_file.is_file() {
        let existing_gpg_id = read_to_string(gpg_id_file.clone())
            .map_err(|err| format!("Error reading gpg id file - {err}"))?;
        if !existing_gpg_id.is_empty() {
            return Err(format!(
                "Error initializing - existing gpg id found: {existing_gpg_id}\nDelete existing id first to re-initialize: rm {}", gpg_id_file.display()
            ));
        }
    } else if !totp_dir.exists() {
        println!("totp dir {:?}", totp_dir);
        create_dir(totp_dir).map_err(|err| format!("Error creating totp dir - {err}"))?;
    }
    write(gpg_id_file, gpg_id).map_err(|err| format!("Error writing gpg id file - {err}"))?;
    Ok(())
}

/// Read GPG id from the given totpc directory.
fn read_gpg_id(totp_dir: &Path) -> Result<String, String> {
    read_to_string(totp_dir.join(GPG_ID_FILE_NAME))
        .map(|gpg_id| gpg_id.trim().to_string())
        .map_err(|err| format!("Error reading gpg id - {err}"))
}

/// List all stored identifiers in the given directory.
pub fn list_identifiers(totp_dir: &Path) -> Result<Vec<String>, String> {
    let files_in_dir = totp_dir
        .read_dir()
        .map_err(|err| format!("Error reading dir {} - {err}", totp_dir.display()))?;
    let mut identifiers: Vec<String> = files_in_dir
        .filter_map(|entry_result| match entry_result {
            Err(err) => {
                eprintln!("Error reading dir - {err}");
                None
            }
            Ok(entry) => {
                if entry
                    .path()
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|ext| ext == GPG_FILE_EXTENSION)
                    .unwrap_or_else(|| false)
                {
                    entry
                        .file_name()
                        .to_str()
                        .map(|file_name| file_name.replace(&format!(".{GPG_FILE_EXTENSION}"), ""))
                } else {
                    None
                }
            }
        })
        .collect();
    identifiers.sort();
    Ok(identifiers)
}

/// Encrypt and store key to file under name <identifier> in given directory.
pub fn write_encrypted_key_to_file(
    gpg_home_dir: &Path,
    totp_dir: &Path,
    identifier: &str,
    key: &str,
) -> Result<(), String> {
    let gpg_id = read_gpg_id(totp_dir)?;
    let new_file_name = format!("{identifier}.{GPG_FILE_EXTENSION}");
    let new_file_path = totp_dir.join(new_file_name);
    let mut gpg_cmd = Command::new(GPG_COMMAND)
        .arg("--homedir")
        .arg(gpg_home_dir)
        .arg("--encrypt")
        .arg("--recipient")
        .arg(gpg_id)
        .arg("-o")
        .arg(&new_file_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Error running encryption command {GPG_COMMAND} - {err}"))?;
    match gpg_cmd.stdin.take() {
        None => return Err("Error inputting key to encrypt command".to_string()),
        Some(mut stdin) => {
            stdin
                .write(key.as_bytes())
                .map_err(|err| format!("Error inputting key to encryption command - {err}"))?;
        }
    };
    let exit_status = gpg_cmd.wait().map_err(|err| err.to_string())?;
    if !exit_status.success() {
        return Err("Error writing encryted key to file".to_string());
    }
    Ok(())
}

/// Decrypt encrypted key from file with name <identifier> in given directory.
pub fn read_decrypted_key_from_file(
    gpg_home_dir: &Path,
    totp_dir: &Path,
    identifier: &str,
) -> Result<Option<String>, String> {
    let gpg_id = read_gpg_id(totp_dir)?;
    let file_name = format!("{identifier}.{GPG_FILE_EXTENSION}");
    let file_path = totp_dir.join(file_name);
    if !file_path.is_file() {
        return Ok(None);
    }
    let output = Command::new(GPG_COMMAND)
        .arg("--homedir")
        .arg(gpg_home_dir)
        .arg("--recipient")
        .arg(gpg_id)
        .arg("--decrypt")
        .arg(&file_path)
        .stdout(Stdio::piped())
        .output()
        .map_err(|err| format!("Error running encryption command {GPG_COMMAND} - {err}"))?;
    if !output.stderr.is_empty() {
        let err = String::from_utf8(output.stderr)
            .map_err(|err| format!("Error parsing {GPG_COMMAND} error message - {err}"))?;
        eprintln!("Reading decrypted key from file - {}", err);
    }
    let decrypted_key = String::from_utf8(output.stdout)
        .map_err(|err| format!("Error reading decrypted key from file - {}", err))?;
    Ok(Some(decrypted_key))
}

/// Delete file with name <identifier> in given directory.
pub fn delete_key_file(totp_dir: &Path, identifier: &str) -> Result<(), String> {
    let file_name = format!("{identifier}.{GPG_FILE_EXTENSION}");
    let file_path = totp_dir.join(file_name);
    remove_file(file_path).map_err(|err| format!("Error deleting key - {err}"))
}

#[cfg(test)]
mod tests {
    use crate::{
        file::{
            init, list_identifiers, read_decrypted_key_from_file, write_encrypted_key_to_file,
            GPG_FILE_EXTENSION,
        },
        TOTP_DIR_NAME,
    };
    use std::{
        fs::{create_dir, read_to_string, write, OpenOptions},
        io::{ErrorKind, Write},
        path::Path,
        process::{Command, Stdio},
    };
    use tempfile::{NamedTempFile, TempDir};

    use super::{delete_key_file, GPG_COMMAND, GPG_ID_FILE_NAME};

    const PASSPHRASE: &str = "abc";

    fn generate_temp_gpg_key_pair(dir: &Path, gpg_id: &str) {
        let config = format!(
            "
            Key-Type: RSA
            Key-Length: 1024
            Subkey-Type: RSA
            Subkey-Length: 1024
            Name-Real: {gpg_id}
            Name-Email: joe@foo.bar
            Expire-Date: 0
            Passphrase: {PASSPHRASE}
        "
        );
        let mut config_file = NamedTempFile::new_in(&dir).unwrap();
        config_file.write_all(config.as_bytes()).unwrap();

        let generate_key_pair_output = Command::new(GPG_COMMAND)
            .arg("--homedir")
            .arg(dir)
            .arg("--batch")
            .arg("--generate-key")
            .arg(config_file.path())
            .output()
            .unwrap();
        if !generate_key_pair_output.status.success() {
            panic!("key generation failed");
        }
    }

    #[test]
    fn init_fails_when_gpg_id_exists() {
        let dir = TempDir::new().unwrap();
        let totp_dir = dir.path().join(TOTP_DIR_NAME);
        create_dir(&totp_dir).unwrap();
        let gpg_id_file_path = totp_dir.join(GPG_ID_FILE_NAME);
        let gpg_id = "test_id";
        write(gpg_id_file_path.clone(), gpg_id).unwrap();

        let result = init(&totp_dir, gpg_id);

        assert!(matches!(result, Err(err) if err.contains(gpg_id) ));
    }

    #[test]
    fn init_writes_file_with_gpg_id() {
        let dir = TempDir::new().unwrap();
        let gpg_id = "test_id";
        let path = dir.into_path().join(TOTP_DIR_NAME);

        init(&path, gpg_id).unwrap();

        let gpg_id_file_path = path.join(GPG_ID_FILE_NAME);
        let actual_content = read_to_string(gpg_id_file_path).unwrap();
        assert_eq!(actual_content, gpg_id);
    }

    #[test]
    fn list_outputs_all_identifiers() {
        let dir = TempDir::new().unwrap();
        let dir_path = dir.path();
        let totp_dir = dir_path.join(TOTP_DIR_NAME);
        create_dir(&totp_dir).unwrap();
        // create gpg id file
        let gpg_id = "Test Man";
        init(&totp_dir, gpg_id).unwrap();
        generate_temp_gpg_key_pair(&dir_path, gpg_id);
        let identifier_1 = "test_id_100";
        let identifier_2 = "test_id_1";
        let key_1 = "test_key_1";
        let key_2 = "test_key_2";
        write_encrypted_key_to_file(&dir_path, &totp_dir, identifier_1, key_1).unwrap();
        write_encrypted_key_to_file(&dir_path, &totp_dir, identifier_2, key_2).unwrap();

        let identifier_list = list_identifiers(&totp_dir).unwrap();

        // list should be ordered by identifier, ascending
        assert_eq!(identifier_list, vec![identifier_2, identifier_1]);
    }

    #[test]
    fn key_is_written_to_encrypted_file() {
        let dir = TempDir::new().unwrap();
        let dir_path = dir.into_path();
        let totp_dir = dir_path.join(TOTP_DIR_NAME);
        create_dir(&totp_dir).unwrap();
        // create gpg id file
        let gpg_id = "Test Man";
        init(&totp_dir, gpg_id).unwrap();
        generate_temp_gpg_key_pair(&dir_path, gpg_id);

        // encrypt key and write to file
        let identifier = "test_identifier";
        let key = "1234567890";
        write_encrypted_key_to_file(&dir_path, &totp_dir, identifier, key).unwrap();

        // decrypt from encrypted file
        let encrypted_file_name = format!("{identifier}.{GPG_FILE_EXTENSION}");
        let encrypted_file_path = totp_dir.join(encrypted_file_name);
        let output = Command::new(GPG_COMMAND)
            .arg("--homedir")
            .arg(dir_path)
            .arg("--decrypt")
            .arg("--recipient")
            .arg(gpg_id)
            .arg("--pinentry-mode")
            .arg("loopback")
            .arg("--passphrase")
            .arg(PASSPHRASE)
            .arg(encrypted_file_path)
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        let decrypted_key = String::from_utf8(output.stdout).unwrap();

        assert_eq!(decrypted_key, key);
    }

    #[test]
    fn key_file_is_deleted() {
        let dir = TempDir::new().unwrap();
        let dir_path = dir.into_path();
        let totp_dir = dir_path.join(TOTP_DIR_NAME);
        create_dir(&totp_dir).unwrap();
        // create gpg id file
        let gpg_id = "Test Man";
        init(&totp_dir, gpg_id).unwrap();
        generate_temp_gpg_key_pair(&dir_path, gpg_id);
        let identifier = "test_identifier";
        let key = "1234567890";
        write_encrypted_key_to_file(&dir_path, &totp_dir, identifier, key).unwrap();

        delete_key_file(&totp_dir, identifier).unwrap();

        let file_name = format!("{identifier}.{GPG_FILE_EXTENSION}");
        let file_path = dir_path.join(TOTP_DIR_NAME).join(file_name);
        let result = OpenOptions::new().read(true).open(file_path);
        assert!(matches!(result, Err(err) if err.kind() == ErrorKind::NotFound));
    }

    #[test]
    #[ignore = "requires manual input"]
    fn read_key_from_file_and_decrypt_manual() {
        let dir = TempDir::new().unwrap();
        let dir_path = dir.into_path();
        let totp_dir = dir_path.join(TOTP_DIR_NAME);
        create_dir(&totp_dir).unwrap();
        // create gpg id file
        let gpg_id = "Test Man";
        init(&totp_dir, gpg_id).unwrap();
        generate_temp_gpg_key_pair(&dir_path, gpg_id);
        // encrypt key and write to file
        let identifier = "test_identifier";
        let key = "1234567890";
        write_encrypted_key_to_file(&dir_path, &totp_dir, identifier, key).unwrap();

        let decrypted_key = read_decrypted_key_from_file(&dir_path, &totp_dir, identifier)
            .unwrap()
            .unwrap();
        assert_eq!(decrypted_key, key);
    }
}
