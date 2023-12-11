fn_prologue.o`fn_prologue::prolog:
    0x102cd7c68 <+0>:   sub    sp, sp, #0x180
    0x102cd7c6c <+4>:   stp    x28, x27, [sp, #0x160]
    0x102cd7c70 <+8>:   stp    x29, x30, [sp, #0x170]
    0x102cd7c74 <+12>:  add    x29, sp, #0x170
    0x102cd7c78 <+16>:  str    w1, [sp, #0x8]
    0x102cd7c7c <+20>:  str    w2, [sp, #0xc]
->  0x102cd7c80 <+24>:  stur   w0, [x29, #-0xa0]
    0x102cd7c84 <+28>:  stur   w1, [x29, #-0x9c]
    0x102cd7c88 <+32>:  stur   w2, [x29, #-0x98]
    0x102cd7c8c <+36>:  bl     0x102cd7ee0               ; fn_prologue::dice at fn_prologue.rs:14
    0x102cd7c90 <+40>:  mov    x8, x0
    0x102cd7c94 <+44>:  ldr    w0, [sp, #0x8]
    0x102cd7c98 <+48>:  str    w8, [sp, #0x10]
    0x102cd7c9c <+52>:  str    w1, [sp, #0x14]
    0x102cd7ca0 <+56>:  bl     0x102cd7ee0               ; fn_prologue::dice at fn_prologue.rs:14
    0x102cd7ca4 <+60>:  mov    x8, x0
    0x102cd7ca8 <+64>:  ldr    w0, [sp, #0xc]
    0x102cd7cac <+68>:  str    w8, [sp, #0x18]
    0x102cd7cb0 <+72>:  str    w1, [sp, #0x1c]
    0x102cd7cb4 <+76>:  bl     0x102cd7ee0               ; fn_prologue::dice at fn_prologue.rs:14
    0x100217edc <+628>: ret    