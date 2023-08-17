.pushsection .head.text, "ax"

.global _start
_start:
	.option push
	.option norelax
	la gp, __global_pointer$
	.option pop

	li sp, 0x80400000

	la a0, KERNEL_ROOT_PAGE_TABLE
	li t0, 0x200000CF
	sd t0, 16(a0)
	srli a0, a0, 12
	li t0, 0x8000000000000000
	or t0, a0, t0
	csrw satp, t0
	sfence.vma

	tail main

.section .bss

.balign 4096
KERNEL_ROOT_PAGE_TABLE:
  .zero 4096

.popsection
