const BIN: &str = "totp";

mod save {
    use assert_cmd::prelude::*;
    use std::process::Command;
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
}
