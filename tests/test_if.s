section .text
        extern snek_error
        global our_code_starts_here
        our_code_starts_here:
 ;Plus
mov rax, 8
;Expr: Num(4)
;Check: Num
test rax, 1
je ok_2
mov rdi, 1
call snek_error
ok_2:
mov [rsp - 16], rax
mov rax, 10
;Expr: Num(5)
;Check: Num
test rax, 1
je ok_3
mov rdi, 1
call snek_error
ok_3:
add rax, [rsp - 16]
;Check: Overflow
jo err_5
jmp ok_4
err_5:
mov rdi, 2
call snek_error
ok_4:
;Expr: BinOp(Plus, Num(4), Num(5))
;Check: Bool
cmp rax, 1
je ok_6
cmp rax, 3
je ok_6
mov rdi, 1
call snek_error
ok_6:
cmp rax, 3
jne else_0
mov rax, 3
;Expr: Bool(true)
jmp end_1
else_0:
mov rax, 1
;Expr: Bool(false)
end_1:
;Expr: If(BinOp(Plus, Num(4), Num(5)), Bool(true), Bool(false)) ret
