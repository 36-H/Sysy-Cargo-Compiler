  .text
  .globl main
main:
  addi sp, sp, -16
.Lentry_2:
  j .L0
.L0:
  li t0, 1
  bnez t0, .Lif_then_4
  j .Lif_else_1
.Lif_then_4:
  li t0, 1
  sw t0, 8(sp)
  j .Lend_5
.L7:
  j .Lif_end_3
.Lif_else_1:
  j .Lif_end_3
.Lif_end_3:
  li t0, 0
  sw t0, 8(sp)
  j .Lend_5
.L6:
  j .Lend_5
.Lend_5:
  lw t0, 8(sp)
  sw t0, 12(sp)
  lw a0, 12(sp)
  addi sp, sp, 16
  ret

