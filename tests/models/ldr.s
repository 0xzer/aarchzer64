    mov x1, sp
    mov x2, #5000
    str x2, [x1]
    mov x3, #0x2710
    str x3, [x1, #8]
    mov x0, #15000
    str x0, [x1, #16]
    ldr x4, [x1]
    ldr x5, [x1, #8]
    ldr x6, [x1, #16]
    mov x2, #2
    ldr x7, [x1, x2, LSL #3]
    mov x8, x1
    add x1, x1, x2, LSL #3
    str x1, [x8, #8]
    ldr w10, [x1]