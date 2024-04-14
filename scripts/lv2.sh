docker run -it --rm -v /root/workspace/sysy-cargo:/root/compiler maxxing/compiler-dev \
  autotest -riscv -s lv1 /root/compiler
