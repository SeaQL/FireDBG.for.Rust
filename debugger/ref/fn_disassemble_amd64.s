fn_prologue.o`fn_prologue::prolog:
    0x563890a01ca0 <+0>:   subq   $0x188, %rsp              ; imm = 0x188 
    0x563890a01ca7 <+7>:   movl   %esi, 0x8(%rsp)
    0x563890a01cab <+11>:  movl   %edx, 0xc(%rsp)
->  0x563890a01caf <+15>:  movl   %edi, 0xf4(%rsp)
    0x563890a01cb6 <+22>:  movl   %esi, 0xf8(%rsp)
    0x563890a01cbd <+29>:  movl   %edx, 0xfc(%rsp)
    0x563890a01cc4 <+36>:  callq  0x563890a01f70            ; fn_prologue::dice at fn_prologue.rs:14
    0x563890a01cc9 <+41>:  movl   0x8(%rsp), %edi
    0x563890a01ccd <+45>:  movl   %eax, 0x10(%rsp)
    0x563890a01cd1 <+49>:  movl   %edx, 0x14(%rsp)
    0x563890a01cd5 <+53>:  callq  0x563890a01f70            ; fn_prologue::dice at fn_prologue.rs:14
    0x563890a01cda <+58>:  movl   0xc(%rsp), %edi
    0x563890a01cde <+62>:  movl   %eax, 0x18(%rsp)
    0x563890a01ce2 <+66>:  movl   %edx, 0x1c(%rsp)
    0x563890a01ce6 <+70>:  callq  0x563890a01f70            ; fn_prologue::dice at fn_prologue.rs:14
    0x563890a01f64 <+708>: retq   