.pushsection .head.text, "ax"
.global _start
_start:
	.option push
	.option norelax
	la gp, __global_pointer$
	.option pop

	li sp, 0x80400000
	j main
.popsection
