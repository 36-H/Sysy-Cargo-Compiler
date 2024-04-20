#!/bin/sh
docker run -it --rm -v /root/workspace/Sysy-Cargo-Compiler:/root/compiler \
  autotest -koopa -s lv6 /root/compiler
