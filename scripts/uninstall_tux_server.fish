#!/usr/bin/fish


function uninstall_tux_server
    if test -d $HOME/.tux-server
        rm -rf $HOME/.tux-server
    end
end

uninstall_tux_server
