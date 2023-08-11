.pushsection .head.text, "ax"
.global _start
_start:
	li sp, 0x80400000
	j main
.popsection
