#!/bin/zsh

function hop2_chpwd () {
    ~/projects/hop2/target/release/main --add "$(pwd)" >/dev/null &!;
}

function h () {
    cmd=$(~/projects/hop2/target/release/main --dir "$@");
    cd "${cmd}";
}

typeset -gaU chpwd_functions 
chpwd_functions+=hop2_chpwd



