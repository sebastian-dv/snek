section .text
        extern snek_error
        global our_code_starts_here
        our_code_starts_here:
 ;Plus
mov rax, rdi
;Expr: Input
;Check: Num
test rax, 1
je ok_0
mov rdi, 1
call snek_error
ok_0:
mov [rsp - 16], rax
mov rax, 10
;Expr: Num(5)
;Check: Num
test rax, 1
je ok_1
mov rdi, 1
call snek_error
ok_1:
add rax, [rsp - 16]
;Check: Overflow
jo err_3
jmp ok_2
err_3:
mov rdi, 2
call snek_error
ok_2:
;Expr: BinOp(Plus, Input, Num(5)) ret
