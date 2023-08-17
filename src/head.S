# Initialization code executed before main.

# Use a separate section so that the linker script can ensure it's the first section.
.pushsection .head.text, "ax"

# Entry point.
.global _start
_start:
	# Initialize gp.
	.option push
	.option norelax
	la gp, __global_pointer$
	.option pop

	# Initialize stack.
	la sp, stack_end

	# Set up initial page table.
	la a0, KERNEL_ROOT_PAGE_TABLE
	# Identity map kernel.
	li t0, 0x200000CF
	sd t0, 16(a0)
	# Write PPN of page table and address translation mode to satp, enabling the MMU.
	srli a0, a0, 12
	li t0, 0x8000000000000000
	or t0, a0, t0
	csrw satp, t0
	sfence.vma

	# Call main.
	tail main

.section .bss

# Top level kernel page table.
.balign 4096
KERNEL_ROOT_PAGE_TABLE:
  .zero 4096

# 1 MiB stack.
.zero 1048576
stack_end:

.popsection
