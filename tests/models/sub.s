	mov x0, #1
	mov x1, #2
	sub x2, x0, x1
	mov x1, #10
	mov x3, 13
	sub x1, x1, x2
	mov x5, 16
	mov x19, 18
	sub x1, x19, x5
	sub x19, x1, x5
	mov w6, #0xd
	mov w7, 0xf
	sub w8, w6, w7
	sub w8, w8, 10, LSL #3
	sub x19, x1, x19, LSL #5
	sub x19, x19, x2, ROR #6
	sub x19, x2, x19, LSR #24
	mov x18, x19
    lsr x19, x18, 0x10