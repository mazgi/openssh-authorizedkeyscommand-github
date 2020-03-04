# OpenSSH AuthorizedKeysCommand GitHub

![GitHub Actions status for "test-all"](https://github.com/mazgi/openssh-authorizedkeyscommand-github/workflows/test-all/badge.svg)

Get your SSH Public Key from GitHub.

```console
> openssh-authorizedkeyscommand-github --help
OpenSSH AuthorizedKeysCommand GitHub

Usage:
  openssh-authorizedkeyscommand-github --user=<user>
  openssh-authorizedkeyscommand-github (--help | --version)

Options:
  --user=<user>   GitHub user.
  -V, --version   Show version.
  -h, --help      Show this screen.
```

## How to use

1. Place the binary in your `$PATH`.

2. Update `/etc/ssh/sshd_config`.

```ssh_config
AuthorizedKeysCommand /usr/local/bin/openssh-authorizedkeyscommand-github --user %u
```

3. Restart SSH server.
