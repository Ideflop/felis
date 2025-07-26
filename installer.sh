#!/usr/bin/bash

set -eu -o pipefail


if [ $# -eq 0 ]; then
    printf "You need to add a path where the binary will be and optionaly a list of aliases for the binary\n installer.sh <Storage Path> <Aliases separaed with space ...>\n"
    exit 1
fi

printf "felis is going to be compiled\n";
sleep 1.5

path="$1"

if cargo build --release ; then

    cp target/release/felis "$path"
    shift
    for ali in "$@"; do
        ln -s "$path"/felis "$path"/"$ali"
    done
    printf "try to run felis to check if it install\n";
else
    printf "
    Unable to run 'cargo build --release'
    If you just installed Rust then try to reload the terminal
    else if Rust ins't install you can download it at :
    www.rust-lang.org\n"
fi
