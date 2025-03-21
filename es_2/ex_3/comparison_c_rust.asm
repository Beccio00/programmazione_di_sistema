
/* C/C++ */
mul(int):
        push    rbp
        mov     rbp, rsp
        mov     DWORD PTR [rbp-4], edi
        mov     eax, DWORD PTR [rbp-4]
        imul    eax, eax
        pop     rbp
        ret



/* RUST */
mul:
        push    rax
        mov     dword ptr [rsp + 4], edi
        imul    edi, edi
        mov     dword ptr [rsp], edi
        seto    al
        jo      .LBB0_2
        mov     eax, dword ptr [rsp]
        pop     rcx
        ret
.LBB0_2:
        lea     rdi, [rip + .L__unnamed_1]
        mov     rax, qword ptr [rip + core::panicking::panic_const::panic_const_mul_overflow::hf1c8947532f8c86b@GOTPCREL]
        call    rax

.L__unnamed_2:
        .ascii  "/app/example.rs"

.L__unnamed_1:
        .quad   .L__unnamed_2
        .asciz  "\017\000\000\000\000\000\000\000\013\000\000\000\005\000\000"
