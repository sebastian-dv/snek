section .text
        extern snek_error
        global our_code_starts_here
        our_code_starts_here:
 ;Times
mov rax, 9223372036854775806
;Expr: Num(4611686018427387903)
;Check: Num
test rax, 1
je ok_0
mov rdi, 1
call snek_error
ok_0:
sar rax, 1
mov [rsp - 16], rax
mov rax, 4
;Expr: Num(2)
;Check: Num
test rax, 1
je ok_1
mov rdi, 1
call snek_error
ok_1:
imul rax, [rsp - 16]
;Check: Overflow
jo err_3
jmp ok_2
err_3:
mov rdi, 2
call snek_error
ok_2:
;Expr: BinOp(Times, Num(4611686018427387903), Num(2)) ret
