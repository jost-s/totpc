use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

pub const DELIMITER: &str = " ";

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    identifier: String,
    key: String,
}

impl Entry {
    fn new(identifier: &str, key: &str) -> Entry {
        Self {
            identifier: identifier.to_string(),
            key: key.to_string(),
        }
    }

    fn to_string(self) -> String {
        format!("{}{}{}\n", self.identifier, DELIMITER, self.key)
    }
}

impl TryFrom<String> for Entry {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.split_once(DELIMITER) {
            None => return Err(format!("Error - missing identifier in entry: {value}")),
            Some((identifier, key)) => Ok(Entry::new(identifier, key)),
        }
    }
}

pub fn ensure_file_exists(path: &Path) -> Result<(), String> {
    if path.is_file() {
        return Ok(());
    }
    File::create(path)
        .map(|_| ())
        .map_err(|error| format!("Error reading file - {error}"))
}

pub fn read_key_from_file(identifier: &str, path: &Path) -> Result<Option<String>, String> {
    let maybe_key = find_entry_in_file(identifier, path)?.and_then(|entry| Some(entry.key));
    Ok(maybe_key)
}

pub fn identifier_exists_in_file(identifier: &str, path: &Path) -> Result<bool, String> {
    Ok(find_entry_in_file(identifier, path)?.is_some())
}

pub fn write_key_to_file(identifier: &str, key: &str, path: &Path) -> Result<(), String> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| format!("Error opening file with write access - {error}"))?;
    let mut writer = BufWriter::new(file);
    let entry = Entry::new(identifier, key);
    writer
        .write_all(entry.to_string().as_bytes())
        .map_err(|error| format!("Error writing to file - {error}"))
}

pub fn list_identifiers(path: &Path) -> Result<Vec<String>, String> {
    let file = File::open(path).map_err(|error| format!("Error reading file - {error}"))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for maybe_line in reader.lines() {
        match maybe_line {
            Err(error) => return Err(format!("Error reading file - {error}")),
            Ok(line) => {
                let entry = Entry::try_from(line)?;
                lines.push(entry.identifier);
            }
        }
    }
    lines.sort();
    Ok(lines)
}

pub fn update_key_in_file(identifier: &str, key: &str, path: &Path) -> Result<(), String> {
    let updated_lines = {
        let file = File::open(path).map_err(|error| format!("Error updating entry - {error}"))?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for maybe_line in reader.lines() {
            match maybe_line {
                Err(error) => return Err(format!("Error updating entry - {error}")),
                Ok(line) => {
                    let mut entry = Entry::try_from(line)?;
                    if entry.identifier == identifier {
                        entry.key = key.to_string();
                    }
                    lines.push(entry.to_string())
                }
            }
        }
        lines
    };
    write_lines_to_file(updated_lines, path)
}

pub fn delete_key_from_file(identifier: &str, path: &Path) -> Result<(), String> {
    let filtered_lines = {
        let file = File::open(path).map_err(|error| format!("Error deleting entry - {error}"))?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for maybe_line in reader.lines() {
            match maybe_line {
                Err(error) => return Err(format!("Error deleting entry - {error}")),
                Ok(line) => {
                    let entry = Entry::try_from(line)?;
                    if entry.identifier != identifier {
                        lines.push(entry.to_string());
                    }
                }
            }
        }
        lines
    };
    write_lines_to_file(filtered_lines, path)
}

fn write_lines_to_file(lines: Vec<String>, path: &Path) -> Result<(), String> {
    let file = File::create(path).map_err(|error| format!("Error writing entry - {error}"))?;
    let mut writer = BufWriter::new(file);
    let all_lines = lines.join("");
    writer
        .write_all(all_lines.as_bytes())
        .map_err(|error| format!("Error writing entry - {error}"))
}

fn find_entry_in_file(identifier: &str, path: &Path) -> Result<Option<Entry>, String> {
    let file = File::open(path).map_err(|error| format!("Error reading file - {error}"))?;
    let reader = BufReader::new(file);
    let read_lines = reader.lines().filter_map(|line| match line {
        Ok(l) => Some(l),
        Err(error) => {
            eprintln!("Error reading line: {error}");
            None
        }
    });
    for line in read_lines {
        let entry = Entry::try_from(line)?;
        if entry.identifier == identifier {
            return Ok(Some(entry));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::file::{
        delete_key_from_file, find_entry_in_file, identifier_exists_in_file, list_identifiers,
        read_key_from_file, update_key_in_file, write_key_to_file, Entry, DELIMITER,
    };
    use std::io::{BufRead, BufReader, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn write() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let key = "1234567890";
        write_key_to_file(identifier, key, file.path()).unwrap();

        let mut lines = BufReader::new(file).lines().map(|line| line.unwrap());
        let actual_entry = Entry::try_from(lines.next().unwrap()).unwrap();
        let expected_entry = Entry::new(identifier, key);
        assert_eq!(actual_entry, expected_entry);
    }

    #[test]
    fn read() {
        let mut file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let expected_key = "1234567890";

        file.write_all(format!("{identifier}{DELIMITER}{expected_key}").as_bytes())
            .unwrap();

        let actual_key = read_key_from_file(identifier, file.path())
            .unwrap()
            .unwrap();
        assert_eq!(expected_key, actual_key);
    }

    #[test]
    fn read_non_existing_identifier() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "test_site";

        let actual_result = read_key_from_file(identifier, file.path()).unwrap();

        assert!(actual_result.is_none());
    }

    #[test]
    fn write_and_read() {
        let file = NamedTempFile::new().unwrap();
        let identifier_1 = "test_id_1";
        let identifier_2 = "test_id_2";
        let expected_key_1 = "test_key_1";
        let expected_key_2 = "test_key_2";

        write_key_to_file(identifier_1, expected_key_1, file.path()).unwrap();
        write_key_to_file(identifier_2, expected_key_2, file.path()).unwrap();

        let actual_key_1 = read_key_from_file(identifier_1, file.path())
            .unwrap()
            .unwrap();
        assert_eq!(actual_key_1, expected_key_1);
        let actual_key_2 = read_key_from_file(identifier_2, file.path())
            .unwrap()
            .unwrap();
        assert_eq!(actual_key_2, expected_key_2);
    }

    #[test]
    fn list() {
        let file = NamedTempFile::new().unwrap();
        let identifier_1 = "test_id_1";
        let identifier_2 = "test_id_2";
        let expected_key_1 = "test_key_1";
        let expected_key_2 = "test_key_2";
        write_key_to_file(identifier_2, expected_key_2, file.path()).unwrap();
        write_key_to_file(identifier_1, expected_key_1, file.path()).unwrap();

        let identifier_list = list_identifiers(file.path()).unwrap();
        // list should be ordered by identifier, ascending
        assert_eq!(identifier_list[0], identifier_1);
        assert_eq!(identifier_list[1], identifier_2);
    }

    #[test]
    fn update_entry() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "id";
        let key = "key";
        let updated_key = "key_updated";
        write_key_to_file(identifier, key, file.path()).unwrap();

        update_key_in_file(identifier, updated_key, file.path()).unwrap();

        let updated_entry = find_entry_in_file(identifier, file.path())
            .unwrap()
            .unwrap();
        assert_eq!(updated_entry, Entry::new(identifier, updated_key));
    }

    #[test]
    fn delete_entry() {
        let file = NamedTempFile::new().unwrap();
        let identifier_1 = "id_1";
        let identifier_2 = "id_2";
        let key_1 = "key_1";
        let key_2 = "key_2";
        write_key_to_file(identifier_1, key_1, file.path()).unwrap();
        write_key_to_file(identifier_2, key_2, file.path()).unwrap();

        delete_key_from_file(identifier_1, file.path()).unwrap();

        let deleted_identifier_exists =
            identifier_exists_in_file(identifier_1, file.path()).unwrap();
        assert_eq!(deleted_identifier_exists, false);
        let identifier_2_exists = identifier_exists_in_file(identifier_2, file.path()).unwrap();
        assert_eq!(identifier_2_exists, true);
    }

    #[test]
    fn identifier_exists() {
        let mut file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let key = "1234567890";
        let entry = Entry::new(identifier, key);
        file.write_all(entry.to_string().as_bytes()).unwrap();

        let identifier_exists = identifier_exists_in_file(identifier, file.path()).unwrap();

        assert_eq!(identifier_exists, true);
    }

    #[test]
    fn identifier_does_not_exist() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "test_site";

        let identifier_exists = identifier_exists_in_file(identifier, file.path()).unwrap();

        assert!(identifier_exists == false);
    }

    #[test]
    fn partial_identifier_does_not_exist() {
        let mut file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let partial_identifier = &identifier[0..2];
        let key = "1234567890";
        let entry = Entry::new(identifier, key);
        file.write_all(entry.to_string().as_bytes()).unwrap();

        let identifier_exists = identifier_exists_in_file(partial_identifier, file.path()).unwrap();

        assert!(identifier_exists == false);
    }
}
