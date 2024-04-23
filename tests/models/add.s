	mov x0, #1
	mov x1, #2
	add x2, x0, x1
	mov x1, #10
	mov x3, 13
	add x1, x2, x1
	mov x5, 16
	mov x19, 18
	add x1, x19, x5
	add x19, x1, x5
	mov w6, #0xd
	mov w7, 0xf
	add w8, w6, w7
	add w8, w8, 10, LSL #3
	add x19, x1, x19, LSL #5
	add x19, x19, x2, ROR #6
	add x19, x2, x19, LSR #24
	lsr x18, x19, #2