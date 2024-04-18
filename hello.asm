  .text
  .globl main
main:
  addi sp, sp, -32
.Lentry_0:
  j .L3
.L3:
  li t0, 10
  sw t0, 24(sp)
  lw t0, 24(sp)
  sw t0, 28(sp)
  lw t0, 28(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 24(sp)
  lw t0, 24(sp)
  sw t0, 8(sp)
  lw t0, 8(sp)
  sw t0, 12(sp)
  j .Lend_1
.L2:
  j .Lend_1
.Lend_1:
  lw t0, 12(sp)
  sw t0, 20(sp)
  lw a0, 20(sp)
  addi sp, sp, 32
  ret

