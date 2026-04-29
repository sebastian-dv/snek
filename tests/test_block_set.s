section .text
        extern snek_error
        global our_code_starts_here
        our_code_starts_here:
 mov rax, 10
;Expr: Num(5)
mov [rsp - 16], rax
;Plus
mov rax, [rsp - 16]
;Expr: Id("x")
;Check: Num
test rax, 1
je ok_0
mov rdi, 1
call snek_error
ok_0:
mov [rsp - 24], rax
mov rax, 2
;Expr: Num(1)
;Check: Num
test rax, 1
je ok_1
mov rdi, 1
call snek_error
ok_1:
add rax, [rsp - 24]
;Check: Overflow
jo err_3
jmp ok_2
err_3:
mov rdi, 2
call snek_error
ok_2:
;Expr: BinOp(Plus, Id("x"), Num(1))
mov [rsp - 16], rax
;Expr: Set("x", BinOp(Plus, Id("x"), Num(1)))
;Expr: Block([Set("x", BinOp(Plus, Id("x"), Num(1)))])
;Expr: Let([("x", Num(5))], Block([Set("x", BinOp(Plus, Id("x"), Num(1)))])) ret
