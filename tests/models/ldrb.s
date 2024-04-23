    mov x1, sp
    mov x2, #0x1234
    str x2, [x1]
    mov x3, #0xAB
    strb x3, [x1, #4]
    mov x2, #1
    lsl x2, x2, #2
    add x1, x1, x2
    ldrb x4, [x1, #4]
    mov x5, #0x5
    add x5, x5, x4
    str x5, [x1, #8]
    ldr x6, [x1]
    ldrb x7, [x1, #8]