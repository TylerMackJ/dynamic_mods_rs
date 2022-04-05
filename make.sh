#!/bin/bash

mkdir -p ${BASH_SOURCE%/*}/build/mods/

(cd ${BASH_SOURCE%/*}/mods/hello_world_mod && cargo build)
cp ${BASH_SOURCE%/*}/mods/hello_world_mod/target/debug/libhello_world_mod.so ${BASH_SOURCE%/*}/build/mods/

(cd ${BASH_SOURCE%/*}/modloader && cargo build)
cp ${BASH_SOURCE%/*}/modloader/target/debug/modloader ${BASH_SOURCE%/*}/build