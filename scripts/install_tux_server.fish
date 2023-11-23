#!/usr/bin/fish

function install_tux_server
    if test ! -d $HOME/.tux-server
        git clone https://github.com/taishingi/tux $HOME/.tux-server || exit 1
        cd $HOME/.tux-server
        cargo run --release
    else
        notify-send "Tux server app is already installed"
    end
end
