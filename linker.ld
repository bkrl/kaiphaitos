OUTPUT_ARCH(riscv)
ENTRY(_start)

PAGE_SIZE = 4K;

SECTIONS {
	. = 0x80200000;

	. = ALIGN(PAGE_SIZE);
	.text : {
		*(.text.entry)
		*(.text .text.*)
	}

	. = ALIGN(PAGE_SIZE);
	.rodata : {
		*(.rodata .rodata.*)
	}

	. = ALIGN(PAGE_SIZE);
	.data : {
		*(.data .data.*)
	}

	.bss : {
		*(.bss .bss.*)
	}
}
