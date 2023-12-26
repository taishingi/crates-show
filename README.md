
- [Toolchain](#toolchain)
- [Shell variables](#shell-variables)
- [Manual install](#manual-install)
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

## Project directory

```shell
cd $HOME/.ls-server
```
