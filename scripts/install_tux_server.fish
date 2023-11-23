#!/usr/bin/fish

function install_tux_server
    if test ! -d $HOME/.tux-server
        git clone https://github.com/taishingi/tux $HOME/.tux-server || exit 1
        cd $HOME/.tux-server
        cargo run --release
    end
    if test ! -f $HOME/.config/systemd/user/tux.service
        cp $HOME/.tux-server/tux.service $HOME/.config/systemd/user/tux.service
        cp $HOME/.tux-server/tux $HOME/bin/
        systemctl --user daemon-reload
        systemctl --user enable tux.service
        systemctl start --user tux.service
    end
end

install_tux_server