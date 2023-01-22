use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

const DELIMITER: &str = " ";

pub fn ensure_file_exists(path: &Path) -> Result<(), String> {
    if path.is_file() {
        return Ok(());
    }
    File::create(path)
        .map(|_| ())
        .map_err(|error| format!("Error reading file - {}", error))
}

fn find_entry_in_file(identifier: &str, path: &Path) -> Result<Option<String>, String> {
    let file = File::open(path).map_err(|error| format!("Error reading file - {}", error))?;
    let reader = BufReader::new(file);
    let maybe_entry = reader
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(l),
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                None
            }
        })
        .find(|line| {
            line.split(DELIMITER)
                .next()
                .and_then(|id| Some(id == identifier))
                .unwrap_or_else(|| false)
        });
    Ok(maybe_entry)
}

pub fn read_key_from_file(path: &Path, identifier: &str) -> Result<Option<String>, String> {
    let maybe_key =
        find_entry_in_file(identifier, path)?.and_then(|line| match line.split(DELIMITER).last() {
            None => None,
            Some(key) => Some(key.to_string()),
        });
    Ok(maybe_key)
}

pub fn identifier_exists_in_file(path: &Path, identifier: &str) -> Result<bool, String> {
    Ok(find_entry_in_file(identifier, path)?.is_some())
}

pub fn write_key_to_file(path: &Path, identifier: &str, key: &str) -> Result<(), String> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| format!("Error reading file - {}", error))?;
    let mut writer = BufWriter::new(file);
    let entry = format!("{}{}{}\n", identifier, DELIMITER, key);
    writer
        .write_all(entry.as_bytes())
        .map_err(|error| format!("Error reading file - {}", error))
}

#[cfg(test)]
mod tests {
    use crate::file::{
        identifier_exists_in_file, read_key_from_file, write_key_to_file, DELIMITER,
    };
    use std::io::{BufRead, BufReader, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn write() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let key = "1234567890";

        write_key_to_file(file.path(), identifier, key).unwrap();

        let mut lines = BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(_, line)| line.unwrap());
        assert!(lines
            .find(|line| line == format!("{}{}{}", identifier, DELIMITER, key).as_str())
            .is_some());
    }

    #[test]
    fn read() {
        let mut file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let expected_key = "1234567890";

        file.write_all(format!("{}{}{}", identifier, DELIMITER, expected_key).as_bytes())
            .unwrap();

        let actual_key = read_key_from_file(file.path(), identifier)
            .unwrap()
            .unwrap();
        assert_eq!(expected_key, actual_key);
    }

    #[test]
    fn read_non_existing_identifier() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "test_site";

        let actual_result = read_key_from_file(file.path(), identifier).unwrap();

        assert!(actual_result.is_none());
    }

    #[test]
    fn write_and_read() {
        let file = NamedTempFile::new().unwrap();
        let identifier_1 = "test_id_1";
        let identifier_2 = "test_id_2";
        let expected_key_1 = "test_key_1";
        let expected_key_2 = "test_key_2";

        write_key_to_file(file.path(), identifier_1, expected_key_1).unwrap();
        write_key_to_file(file.path(), identifier_2, expected_key_2).unwrap();

        let actual_key_1 = read_key_from_file(file.path(), identifier_1)
            .unwrap()
            .unwrap();
        let actual_key_2 = read_key_from_file(file.path(), identifier_2)
            .unwrap()
            .unwrap();

        assert_eq!(actual_key_1, expected_key_1);
        assert_eq!(actual_key_2, expected_key_2);
    }

    #[test]
    fn identifier_exists() {
        let mut file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let key = String::from("1234567890");

        file.write_all(format!("{}{}{}", identifier, DELIMITER, key).as_bytes())
            .unwrap();

        let identifier_exists = identifier_exists_in_file(file.path(), identifier).unwrap();

        assert!(identifier_exists);
    }

    #[test]
    fn identifier_does_not_exist() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "test_site";

        let identifier_exists = identifier_exists_in_file(file.path(), identifier).unwrap();

        assert!(identifier_exists == false);
    }

    #[test]
    fn partial_identifier_does_not_exist() {
        let mut file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let partial_identifier = &identifier[0..2];
        let key = String::from("1234567890");

        file.write_all(format!("{}{}{}", identifier, DELIMITER, key).as_bytes())
            .unwrap();

        let identifier_exists = identifier_exists_in_file(file.path(), partial_identifier).unwrap();

        assert!(identifier_exists == false);
    }
}
