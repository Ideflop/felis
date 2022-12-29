#!/usr/bin/bash

# Installer for felis

set -eu -o pipefail

echo "felis is going to be compiled";
sleep 1.5

if cargo build --release ; then
   echo "felis will be copy to /usr/bin/ therefore it needs the sudo right";
   sudo cp target/release/felis /usr/bin/
   echo "try to run felis to check if it install";
else
   cat << EOF
   Unable to run 'cargo build --release'
   If you just installed Rust then try to reload the terminal
   else if Rust ins't install you can download it at :
   www.rust-lang.org
EOF
fi
