#!/bin/sh
docker run -it --rm -v /root/workspace/Sysy-Cargo-Compiler:/root/compiler maxxing/compiler-dev \
  autotest -koopa -s lv5 /root/compiler