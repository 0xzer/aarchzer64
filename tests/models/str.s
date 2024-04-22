    mov x1, sp
    mov x2, #5000
    str x2, [x1]
    mov x3, #0x2710
    str w3, [x1, #8]
    str x1, [x1, #16]
    mov w4, #15000
    add w4, w4, w3
    mov x2, #0x3
    str w4, [x1, x2, LSL #3]