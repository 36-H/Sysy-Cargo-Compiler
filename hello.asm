  .data
  .globl var
var:
  .zero 4

  .text
  .globl main
main:
  addi sp, sp, -16
.Lentry_1:
  j .L2
.L2:
  la t0, var
  lw t0, 0(t0)
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  sw t0, 8(sp)
  j .Lend_3
.L0:
  j .Lend_3
.Lend_3:
  lw t0, 8(sp)
  sw t0, 0(sp)
  lw a0, 0(sp)
  addi sp, sp, 16
  ret

