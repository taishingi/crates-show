#!/usr/bin/fish

function update_tux_server()
    systemctl stop --user tux.service
    cd $HOME/.tux-server || exit 1
    git pull origin master
    systemctl restart --user tux.service
    cd $TUX_DIR
    ranger
end

update_tux_server
