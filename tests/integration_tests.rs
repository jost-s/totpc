const BIN: &str = "totp";

mod save {
    use assert_cmd::prelude::*;
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    use totp::{ErrorMessage, COMMAND_SAVE};

    use crate::BIN;

    #[test]
    fn without_identifier_fails() {
        let mut command = Command::cargo_bin(BIN).unwrap();
        let command = command.arg(COMMAND_SAVE);

        command.assert().code(1);
        let error = command.unwrap_err();
        assert!(error
            .to_string()
            .contains(ErrorMessage::MissingIdentifier.as_str()));
    }

    #[test]
    #[ignore = "uses actual file"]
    fn without_key_fails() {
        let mut command = Command::cargo_bin(BIN).unwrap();
        let command = command.arg(COMMAND_SAVE).arg("test_identifier");

        command.assert().code(1);
        let error = command.unwrap_err();
        println!("{}", error);
        assert!(error.to_string().contains(ErrorMessage::EmptyKey.as_str()));
    }

    #[test]
    #[ignore = "uses actual file"]
    fn succeeds() {
        let mut command = Command::cargo_bin(BIN).unwrap();
        let mut child = command
            .arg(COMMAND_SAVE)
            .arg("test_identifier")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        // if let Some(mut stdin) = child.stdin.take() {
        //     stdin.write_all(b"ORSXG5A=").unwrap();
        // }
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(b"ORSXG5A=").unwrap();
        let output = child.wait_with_output().unwrap();
        println!("{:?}", output);

        // command.assert().code(1);
        // let error = command.unwrap_err();
        // println!("{error}");
        // assert!(error.to_string().contains(ErrorMessage::EmptyKey.as_str()));
    }
}
