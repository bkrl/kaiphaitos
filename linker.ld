/* QEMU executes from the beginning so this doesn't actually matter. */
ENTRY(_start)

PAGE_SIZE = 4K;
RAM_START = 0x80000000;
/*
 * Physical address where the kernel is loaded.
 * This isn't at the beginning of RAM since the firmware uses that region.
 */
LOAD_ADDRESS = RAM_START + 0x200000;
/* Difference between virtual and physical address of kernel. */
KERNEL_OFFSET = 0xffffffff00000000;
KERNEL_ADDRESS = LOAD_ADDRESS + KERNEL_OFFSET;

SECTIONS {
	. = KERNEL_ADDRESS;
	
	. = ALIGN(PAGE_SIZE);
	/*
	 * QEMU always starts executing from the beginning of the kernel,
	 * so this has to be the first section.
	 */
	.head.text : AT(ADDR(.head.text) - KERNEL_OFFSET){
		*(.head.text)
	}
	.text : AT(ADDR(.text) - KERNEL_OFFSET) {
		*(.text .text.*)
	}
	
	. = ALIGN(PAGE_SIZE);
	.rodata : AT(ADDR(.rodata) - KERNEL_OFFSET) {
		*(.rodata .rodata.*)
	}
	/* This might be useful for debugging? */
	.eh_frame : AT(ADDR(.eh_frame) - KERNEL_OFFSET) {
		KEEP(*(.eh_frame .eh_frame.*))
	}
	
	. = ALIGN(PAGE_SIZE);
	.data : AT(ADDR(.data) - KERNEL_OFFSET) {
		*(.data .data.*)
	}
	.sdata : AT(ADDR(.sdata) - KERNEL_OFFSET) {
		__global_pointer$ = . + 0x800;
		*(.sdata*)
	}

	.got : { *(.got*) }
	
	. = ALIGN(PAGE_SIZE);
	.sbss : AT(ADDR(.sbss) - KERNEL_OFFSET) {
	  *(.sbss .sbss.*)
	}
	. = ALIGN(PAGE_SIZE);
	.bss : AT(ADDR(.bss) - KERNEL_OFFSET) {
		*(.bss .bss.*)
		/* 1MiB stack. */
		. += 1M;
		stack_end = .;
	}

	. = ALIGN(PAGE_SIZE);
	FIRST_ALLOCATABLE_FRAME = .;
	
	/* Copied from GNU ld default script. */
	/* Copyright (C) 2014-2023 Free Software Foundation, Inc.
	 Copying and distribution of this script, with or without modification,
	 are permitted in any medium without royalty provided the copyright
	 notice and this notice are preserved.  */
	/* DWARF debug sections.
	   Symbols in the DWARF debugging sections are relative to the beginning
	   of the section so we begin them at 0.  */
	/* DWARF 1.  */
	.debug          0 : { *(.debug) }
	.line           0 : { *(.line) }
	/* GNU DWARF 1 extensions.  */
	.debug_srcinfo  0 : { *(.debug_srcinfo) }
	.debug_sfnames  0 : { *(.debug_sfnames) }
	/* DWARF 1.1 and DWARF 2.  */
	.debug_aranges  0 : { *(.debug_aranges) }
	.debug_pubnames 0 : { *(.debug_pubnames) }
	/* DWARF 2.  */
	.debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }
	.debug_abbrev   0 : { *(.debug_abbrev) }
	.debug_line     0 : { *(.debug_line .debug_line.* .debug_line_end) }
	.debug_frame    0 : { *(.debug_frame) }
	.debug_str      0 : { *(.debug_str) }
	.debug_loc      0 : { *(.debug_loc) }
	.debug_macinfo  0 : { *(.debug_macinfo) }
	/* SGI/MIPS DWARF 2 extensions.  */
	.debug_weaknames 0 : { *(.debug_weaknames) }
	.debug_funcnames 0 : { *(.debug_funcnames) }
	.debug_typenames 0 : { *(.debug_typenames) }
	.debug_varnames  0 : { *(.debug_varnames) }
	/* DWARF 3.  */
	.debug_pubtypes 0 : { *(.debug_pubtypes) }
	.debug_ranges   0 : { *(.debug_ranges) }
	/* DWARF 5.  */
	.debug_addr     0 : { *(.debug_addr) }
	.debug_line_str 0 : { *(.debug_line_str) }
	.debug_loclists 0 : { *(.debug_loclists) }
	.debug_macro    0 : { *(.debug_macro) }
	.debug_names    0 : { *(.debug_names) }
	.debug_rnglists 0 : { *(.debug_rnglists) }
	.debug_str_offsets 0 : { *(.debug_str_offsets) }
	.debug_sup      0 : { *(.debug_sup) }
	
	.comment  0 : { *(.comment) }
	.symtab   0 : { *(.symtab) }
	.strtab   0 : { *(.strtab) }
	.shstrtab 0 : { *(.shstrtab) }
	
	.riscv.attributes 0 : { *(.riscv.attributes) }
}
