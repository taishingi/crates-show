# tux

> A web application to manage rust projects.

[**Design**](https://www.figma.com/file/LoH2IH17LXFlugXmuuio3o/tux?type=design&node-id=1%3A2&mode=design&t=3ZFpY3ZYs3328J0N-1)

# Installation

```shell
rustup default nightly
```

```shell
echo "set -x TUX_DIR $HOME/RustroverProjects" >> $HOME/.config/fish/config.fish
```

```shell
echo "set -x TUX_EDITOR rustrover" >> $HOME/.config/fish/config.fish
```

```shell
git clone https://github.com/taishingi/tux.git /tmp/tux && cd /tmp/tux
```

```shell
cp tux.service ~/.config/systemd/user/
```

```shell
sudo cp tux /usr/bin/
```

```shell
systemctl enable --user tux.service
```

```shell
systemctl start --user tux.service
```

> Service access url

```http
http://127.0.0.1:3000
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

