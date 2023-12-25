
- [Toolchain](#toolchain)
- [Shell variables](#shell-variables)
- [Manual install](#manual-install)
- [Service access url](#service-access-url)
- [Update](#update)
- [Project directory](#project-directory)
  
## Toolchain

```shell
rustup default nightly
```

## Shell variables 

```shell
export CRATES_DIR=$HOME/Documents/Github
export CRATES_EDITOR=code
export CRATES_PROVIDER=github.com
export CRATES_PROVIDER_USERNAME=taishingi
```

## Manual install

```shell
git clone https://github.com/taishingi/crates-show.git /tmp/crates
cd /tmp/crates
vim crates.service
cp crates.service ~/.config/systemd/user/
sudo cp crates /usr/bin/
systemctl enable --user --now creates.service
```

## Service access url

```bash
vivaldi-stable http://127.0.0.1:3000
```

## Update

```shell
systemctl stop --user tux.service
```

```shell
systemctl start --user tux.service
```

## Project directory

```shell
cd $HOME/.tux-server
```
