#!/bin/bash

mkdir -p ${BASH_SOURCE%/*}/build/mods/

MOD="hello_world_mod"
(cd ${BASH_SOURCE%/*}/mods/$MOD && cargo build)
cp ${BASH_SOURCE%/*}/mods/$MOD/target/debug/*.so ${BASH_SOURCE%/*}/build/mods/

MOD=example_mod
(cd ${BASH_SOURCE%/*}/mods/$MOD && cargo build)
cp ${BASH_SOURCE%/*}/mods/$MOD/target/debug/*.so ${BASH_SOURCE%/*}/build/mods/

(cd ${BASH_SOURCE%/*}/modloader && cargo build)
cp ${BASH_SOURCE%/*}/modloader/target/debug/modloader ${BASH_SOURCE%/*}/build