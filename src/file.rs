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
        .map_err(|error| format!("Error reading file - {error}"))
}

pub fn read_key_from_file(path: &Path, identifier: &str) -> Result<Option<String>, String> {
    let maybe_key =
        find_entry_in_file(identifier, path)?.and_then(|line| match line.split(DELIMITER).last() {
            None => None,
            Some(key) => Some(key.to_string()),
        });
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
    let entry = format!("{}{}{}\n", identifier, DELIMITER, key);
    writer
        .write_all(entry.as_bytes())
        .map_err(|error| format!("Error writing to file - {error}"))
}

pub fn list_identifiers(path: &Path) -> Result<Vec<String>, String> {
    let file = File::open(path).map_err(|error| format!("Error reading file - {error}"))?;
    let reader = BufReader::new(file);
    let mut list = Vec::new();
    for line in reader.lines() {
        match line {
            Err(error) => return Err(format!("Error reading file - {error}")),
            Ok(entry) => match entry.split_once(DELIMITER) {
                None => return Err(format!("Error reading file - {entry}")),
                Some((identifier, _)) => list.push(identifier.to_string()),
            },
        }
    }
    list.sort();
    Ok(list)
}

pub fn update_key_in_file(identifier: &str, key: &str, path: &Path) -> Result<(), String> {
    let updated_lines = {
        let file = File::open(path).map_err(|error| format!("Error updating entry - {error}"))?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        for maybe_line in reader.lines() {
            match maybe_line {
                Err(error) => return Err(format!("Error updating entry - {error}")),
                Ok(entry) => match entry.split_once(DELIMITER) {
                    None => {
                        return Err(format!(
                            "Error updating entry - missing identifier in entry: {entry}"
                        ))
                    }
                    Some((id, _)) => {
                        if id == identifier {
                            let updated_entry = format!("{identifier}{DELIMITER}{key}");
                            lines.push(updated_entry);
                        } else {
                            lines.push(entry)
                        }
                    }
                },
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
                Ok(entry) => match entry.split_once(DELIMITER) {
                    None => {
                        return Err(format!(
                            "Error deleting entry - missing identifier in entry: {entry}"
                        ))
                    }
                    Some((id, _)) => {
                        if id != identifier {
                            lines.push(entry);
                        }
                    }
                },
            }
        }
        lines
    };
    write_lines_to_file(filtered_lines, path)
}

fn write_lines_to_file(lines: Vec<String>, path: &Path) -> Result<(), String> {
    let file = File::create(path).map_err(|error| format!("Error writing entry - {error}"))?;
    let mut writer = BufWriter::new(file);
    let mut all_lines = lines.join("\n");
    // append newline at end of file
    all_lines.push_str("\n");
    writer
        .write_all(all_lines.as_bytes())
        .map_err(|error| format!("Error writing entry - {error}"))
}

fn find_entry_in_file(identifier: &str, path: &Path) -> Result<Option<String>, String> {
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
        match line.split_once(DELIMITER) {
            None => return Err(format!("Error - missing identifier in entry: {line}")),
            Some((id, _)) => {
                if id == identifier {
                    return Ok(Some(line));
                }
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::file::{
        delete_key_from_file, find_entry_in_file, identifier_exists_in_file, list_identifiers,
        read_key_from_file, update_key_in_file, write_key_to_file, DELIMITER,
    };
    use std::io::{BufRead, BufReader, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn write() {
        let file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let key = "1234567890";
        write_key_to_file(identifier, key, file.path()).unwrap();

        let mut lines = BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(_, line)| line.unwrap());
        assert!(lines
            .find(|line| line == format!("{identifier}{DELIMITER}{key}").as_str())
            .is_some());
    }

    #[test]
    fn read() {
        let mut file = NamedTempFile::new().unwrap();
        let identifier = "test_site";
        let expected_key = "1234567890";

        file.write_all(format!("{identifier}{DELIMITER}{expected_key}").as_bytes())
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

        write_key_to_file(identifier_1, expected_key_1, file.path()).unwrap();
        write_key_to_file(identifier_2, expected_key_2, file.path()).unwrap();

        let actual_key_1 = read_key_from_file(file.path(), identifier_1)
            .unwrap()
            .unwrap();
        assert_eq!(actual_key_1, expected_key_1);
        let actual_key_2 = read_key_from_file(file.path(), identifier_2)
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
        // list should ordered by identifier, ascending
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
        assert_eq!(
            updated_entry,
            format!("{identifier}{DELIMITER}{updated_key}")
        );
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
        let key = String::from("1234567890");
        file.write_all(format!("{identifier}{DELIMITER}{key}").as_bytes())
            .unwrap();

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
        let key = String::from("1234567890");
        file.write_all(format!("{identifier}{DELIMITER}{key}").as_bytes())
            .unwrap();

        let identifier_exists = identifier_exists_in_file(partial_identifier, file.path()).unwrap();

        assert!(identifier_exists == false);
    }
}
