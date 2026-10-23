# totpc

[![Test](https://github.com/jost-s/totpc/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/jost-s/totpc/actions/workflows/test.yml)
[![crates.io](https://img.shields.io/crates/v/totpc.svg)](https://crates.io/crates/totpc)
[![License: MIT](https://img.shields.io/crates/l/totpc.svg)](https://github.com/jost-s/totpc)

**totp computer**: a command-line authenticator. It stores your 2FA secrets
encrypted with GPG and computes time-based one-time passwords
([TOTP, RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238)) in the
terminal, so you don't need a phone app.

```console
$ totpc compute github
Current TOTP for github is 492039
```

## Installation

```sh
cargo install totpc
```

You also need [GnuPG](https://www.gnupg.org/) on your `PATH` and a GPG key pair
in `~/.gnupg`. totpc runs on macOS and Linux.

## Quick start

```sh
# 1. Choose the GPG key that encrypts your secrets.
#    To reuse your pass key: cat ~/.password-store/.gpg-id
totpc init you@example.com

# 2. Store a secret. Paste the setup key the service shows you
#    (usually under "can't scan the QR code?").
totpc store github
# Enter key for github:
# JBSW Y3DP EHPK 3PXP

# 3. Get the current code.
totpc compute github
```

You can paste secrets with spaces and in any case.

## Commands

| Command                | Short | Description                                        |
| ---------------------- | ----- | -------------------------------------------------- |
| `init <gpg-id>`        | `i`   | Set the GPG key used to encrypt secrets            |
| `list`                 | `l`   | List all identifiers (runs if no command is given) |
| `store <identifier>`   | `s`   | Encrypt and save a secret read from stdin          |
| `compute <identifier>` | `c`   | Print the current TOTP                             |
| `read <identifier>`    | `r`   | Decrypt and print the stored secret                |
| `delete <identifier>`  |       | Delete the entry                                   |
| `--help`               |       | Show usage                                         |

```console
$ totpc
totp computer
├─ aws
└─ github
```

## Built like `pass`

totpc doesn't just borrow ideas from [pass](https://www.passwordstore.org/). It
uses the same storage mechanism:

|                | pass                          | totpc                  |
| -------------- | ----------------------------- | ---------------------- |
| Store          | `~/.password-store/`          | `~/.totpc/`            |
| Encryption key | `.gpg-id`                     | `.gpg-id`              |
| Entries        | `<name>.gpg`                  | `<identifier>.gpg`     |
| Encryption     | `gpg` + `gpg-agent`           | `gpg` + `gpg-agent`    |

If you initialize totpc with the same GPG key as pass, both tools use the same
`gpg-agent` session. You enter your passphrase once, and you get the password
and then the code without a second prompt:

```sh
pass -c github
totpc c github
```

You can also chain them:

```sh
pass -c github && totpc c github

Or put it in a shell function, and use the same names in both stores:

```sh
login() { pass -c "$1" && totpc compute "$1"; }
```

Every entry is a normal GPG file, so you can read it without totpc
(`gpg -d ~/.totpc/github.gpg`), back it up, or put it under git like a
password store.

## Shell completion

The completions cover commands and stored identifiers.

**Bash**

```sh
curl -fsSL https://raw.githubusercontent.com/jost-s/totpc/main/totpc.bash -o ~/.totpc-completion.bash
echo 'source ~/.totpc-completion.bash' >> ~/.bashrc
```

**Zsh**: source the file after `compinit` runs in your `~/.zshrc`.

```sh
curl -fsSL https://raw.githubusercontent.com/jost-s/totpc/main/_totpc.zsh -o ~/.totpc-completion.zsh
echo 'source ~/.totpc-completion.zsh' >> ~/.zshrc
```

## Compatibility

totpc uses the RFC 6238 defaults: HMAC-SHA1, 6 digits and a 30-second period.
These are the settings Google Authenticator, Authy and most websites use.
Services that need other settings aren't supported yet.

## Security

- Secrets go straight from stdin to `gpg`. totpc never writes them to disk
  unencrypted.
- `read` prints the secret in plain text, for example to move it to another
  device.
- If pass and totpc share a GPG key, one key protects both your password and
  your second factor. If you want the two kept separate, give totpc its own key.

## Alternatives

- [pass-otp](https://github.com/tadfisher/pass-otp) stores secrets inside your
  password store. totpc keeps them in a separate store, so you can use a
  different key, a different machine, or a different backup policy.
- [oathtool](https://www.nongnu.org/oath-toolkit/) computes codes, but it
  doesn't store secrets. You have to supply the secret every time.

## License

MIT
