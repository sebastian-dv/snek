section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        our_code_starts_here:
 ;Main expression
mov rax, 10
;Expr: Num(5)
;Print
mov rdi, rax
push rax
call snek_print
pop rax
;Expr: UnOp(Print, Num(5))
ret
