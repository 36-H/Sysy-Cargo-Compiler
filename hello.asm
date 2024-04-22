  .text
  .globl main
main:
  addi sp, sp, -80
.Lentry_3:
  j .L0
.L0:
  addi t0, sp, 44
  li t1, 0
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 0
  slli t1, t1, 2
  add t0, t0, t1
  sw t0, 76(sp)
  li t0, 1
  lw t1, 76(sp)
  sw t0, 0(t1)
  lw t0, 8(sp)
  li t1, 1
  slli t1, t1, 2
  add t0, t0, t1
  sw t0, 36(sp)
  li t0, 2
  lw t1, 36(sp)
  sw t0, 0(t1)
  lw t0, 8(sp)
  li t1, 2
  slli t1, t1, 2
  add t0, t0, t1
  sw t0, 32(sp)
  li t0, 0
  lw t1, 32(sp)
  sw t0, 0(t1)
  addi t0, sp, 44
  li t1, 1
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  li t1, 0
  slli t1, t1, 2
  add t0, t0, t1
  sw t0, 68(sp)
  li t0, 0
  lw t1, 68(sp)
  sw t0, 0(t1)
  lw t0, 12(sp)
  li t1, 1
  slli t1, t1, 2
  add t0, t0, t1
  sw t0, 16(sp)
  li t0, 0
  lw t1, 16(sp)
  sw t0, 0(t1)
  lw t0, 12(sp)
  li t1, 2
  slli t1, t1, 2
  add t0, t0, t1
  sw t0, 40(sp)
  li t0, 0
  lw t1, 40(sp)
  sw t0, 0(t1)
  addi t0, sp, 44
  li t1, 0
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 2
  slli t1, t1, 2
  add t0, t0, t1
  sw t0, 24(sp)
  lw t0, 24(sp)
  lw t0, 0(t0)
  sw t0, 28(sp)
  lw t0, 28(sp)
  sw t0, 20(sp)
  j .Lend_1
.L2:
  j .Lend_1
.Lend_1:
  lw t0, 20(sp)
  sw t0, 72(sp)
  lw a0, 72(sp)
  addi sp, sp, 80
  ret

