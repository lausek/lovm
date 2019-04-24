#!/bin/bash

VIM_PATH="$HOME/.config/nvim"

echo "syntax manager for loas - lausek, 2019"
echo "script for installation of loas syntax highlighting in vim"

help () {
    echo "INFO: use 'install' for setup or 'uninstall' to revoke changes."
}

install () {
    # copy syntax sheet
    mkdir "$VIM_PATH/syntax" -p
    cp "./loas.vim" "$VIM_PATH/syntax/loas.vim" -v
    # create entry for automatic file detection
    mkdir "$VIM_PATH/ftdetect" -p
    echo "autocmd BufNewFile,BufRead *.loas set filetype=loas" > "$VIM_PATH/ftdetect/loas.vim"
    echo "ftdetect -> $VIM_PATH/ftdetect/loas.vim"
    echo "INFO: operation ended successfully!"
}

uninstall () {
    # TODO: if directory is empty, `rmdir` it
    rm "$VIM_PATH/syntax/loas.vim" -v
    rm "$VIM_PATH/ftdetect/loas.vim" -v
    echo "INFO: operation ended successfully!"
}

case $1 in
    install)
        install
        ;;
    uninstall)
        uninstall
        ;;
    *)
        echo "ERR: unknown command '$1'"
        help
        ;;
esac
