section .text
            extern snek_error
            global our_code_starts_here
            our_code_starts_here:
                    ;Times
mov rax, 3
;Expr: Bool(true)
;Check: Num
test rax, 1
je ok_2
mov rdi, 1
call snek_error
ok_2:
mov [rsp - 16], rax
mov rax, 3
;Expr: Bool(true)
;Check: Num
test rax, 1
je ok_3
mov rdi, 1
call snek_error
ok_3:
imul rax, [rsp - 16]
;Check: Overflow
jo err_1
jmp ok_0
err_1:
mov rdi, 2
call snek_error
ok_0:
;Expr: BinOp(Times, Bool(true), Bool(true))
                    ret
