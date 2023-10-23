# Initialization code executed before main.

# Use a separate section so that the linker script can ensure it's the first section.
.pushsection .head.text, "ax"

# Entry point.
.global _start
_start:
	# Disable gp relaxation since gp isn't initialized yet.
	.option push
	.option norelax

	# Write PPN of page table and address translation mode to satp, enabling the MMU.
	la a0, INIT_PAGE_TABLE
	srli a0, a0, 12
	li t0, 8 << 60  # Sv39 mode
	or t0, a0, t0
	csrw satp, t0
	sfence.vma

	# Absolute jump to the higher half.
	lga ra, 1f
	jr ra
1:

	# Initialize gp.
	la gp, __global_pointer$
	.option pop

	# Initialize stack pointer.
	la sp, stack_end

	# Call main.
	tail main

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

.popsection
