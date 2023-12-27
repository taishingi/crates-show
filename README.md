# Ls

A cargo website to help rust programers.

- [Ls](#ls)
  - [Toolchain](#toolchain)
  - [Dependencies](#dependencies)
  - [Manual install](#manual-install)
  - [Project directory](#project-directory)
  - [Variables](#variables)
    - [Systemd](#systemd)
    - [Bash](#bash)
    - [Fish](#fish)

## Toolchain

```shell
rustup default nightly
```

## Dependencies

- npm
- nodejs
- rustup
- cargo

## Manual install

```shell
git clone https://github.com/taishingi/ls /tmp/ls
cd /tmp/ls
vim ls-server.service
cp ls-server.service $HOME/.config/systemd/user/  
mkdir -p $HOME/bin
cp launch-ls-server $HOME/bin/launch-ls-server
systemctl enable --user --now ls-server.service
```

## Project directory

```shell
cd $HOME/.ls-server
```

## Variables

Variable using by the website to get personals informations.

### Systemd

```ini
Environment="CRATES_DIR=/home/juwishmaster/Documents/GitHub"
Environment="CRATES_EDITOR=code-insiders"
Environment="CRATES_PROVIDER=github.com"
Environment="CRATES_PROVIDER_USERNAME=taishingi"             
```

### Bash

```bash
export CRATES_DIR=$HOME/Documents/Github  # Path to your project
export CRATES_EDITOR=code-insiders        # Your prefers code editor
export CRATES_PROVIDER=github.com         # The site where you can clone crates
export CRATES_PROVIDER_USERNAME=taishingi # The website username to clone crates
```

### Fish

```bash
set -x CRATES_DIR $HOME/Documents/Github  # Path to your project
set -x CRATES_EDITOR code-insiders        # Your prefers code editor
set -x CRATES_PROVIDER github.com         # The site where you can clone crates
set -x CRATES_PROVIDER_USERNAME taishingi # The website username to clone crates
```
