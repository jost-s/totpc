# totp computer

Desktop CLI tool to manage TOTP keys and compute values.

A time-based one-time password ([TOTP](https://datatracker.ietf.org/doc/html/rfc6238#section-4))
is used in 2-factor authentication (2FA) as a second step when logging in to a
user account.

Inspired by ["pass"](https://www.passwordstore.org/), totp comouter provides
commands to generate TOTPs. Keys are stored locally and encrypted through the
[GNU Privacy Guard](https://www.gnupg.org/).
