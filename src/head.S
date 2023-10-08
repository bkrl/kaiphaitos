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

	call init_paging

	# Reinitialize gp and sp now that we've relocated to the higher half.
	.option push
	.option norelax
	la gp, __global_pointer$
	.option pop

	la sp, stack_end

	# Call main.
	tail main

init_paging:
	# Write PPN of page table and address translation mode to satp, enabling the MMU.
	la a0, INIT_PAGE_TABLE
	srli a0, a0, 12
	li t0, 8 << 60  # Sv39 mode
	or t0, a0, t0
	csrw satp, t0
	sfence.vma

	# Adjust the return address to point to the higher half.
	li t0, 0xffffffff00000000
	add ra, ra, t0

	ret

.section .rodata

# Initial page table used to relocate to the higher half.
.balign 4096
INIT_PAGE_TABLE:
	# PTE pointing to the start of RAM where the kernel is.
	#               phys addr            DAGUXWRV
	.set INIT_PTE, (0x80000000 >> 2) | 0b11001111
  .zero 8 * 2
	# Identity map the kernel.
	.8byte INIT_PTE
	.zero 8 * 507
	# Map the kernel to 0xffffffff80000000.
	.8byte INIT_PTE
	.zero 8

.section .bss

# 1 MiB stack.
.zero 1024 * 1024
stack_end:

.popsection
