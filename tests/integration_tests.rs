const BIN: &str = "totp";

use assert_cmd::prelude::*;
use std::process::Command;
use totp::{ErrorMessage, COMMAND_COMPUTE, COMMAND_DELETE, COMMAND_SAVE, COMMAND_UPDATE};

#[test]
fn save_without_identifier_fails() {
    let mut command = Command::cargo_bin(BIN).unwrap();
    let command = command.arg(COMMAND_SAVE);

    command.assert().code(1);
    let error = command.unwrap_err();
    assert!(error
        .to_string()
        .contains(ErrorMessage::MissingIdentifier.as_str()));
}

#[test]
fn read_without_identifier_fails() {
    let mut command = Command::cargo_bin(BIN).unwrap();
    let command = command.arg(COMMAND_DELETE);

    command.assert().code(1);
    let error = command.unwrap_err();
    assert!(error
        .to_string()
        .contains(ErrorMessage::MissingIdentifier.as_str()));
}

#[test]
fn update_without_identifier_fails() {
    let mut command = Command::cargo_bin(BIN).unwrap();
    let command = command.arg(COMMAND_UPDATE);

    command.assert().code(1);
    let error = command.unwrap_err();
    assert!(error
        .to_string()
        .contains(ErrorMessage::MissingIdentifier.as_str()));
}

#[test]
fn compute_without_identifier_fails() {
    let mut command = Command::cargo_bin(BIN).unwrap();
    let command = command.arg(COMMAND_COMPUTE);

    command.assert().code(1);
    let error = command.unwrap_err();
    assert!(error
        .to_string()
        .contains(ErrorMessage::MissingIdentifier.as_str()));
}
