  .text
  .globl main
main:
  addi sp, sp, -16
.Lentry_0:
  j .L3
.L3:
  li t0, 0
  li t1, 0
  xor t0, t0, t1
  seqz t0, t0
  sw t0, 12(sp)
  lw t0, 12(sp)
  sw t0, 8(sp)
  j .Lend_2
.L1:
  j .Lend_2
.Lend_2:
  lw t0, 8(sp)
  sw t0, 4(sp)
  lw a0, 4(sp)
  addi sp, sp, 16
  ret

