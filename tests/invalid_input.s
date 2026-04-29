section .text
        extern snek_error
        global our_code_starts_here
        our_code_starts_here:
 ;Equal
mov rax, 1
;Expr: Bool(false)
mov rcx, rax
mov [rsp - 16], rax
mov rax, rdi
;Expr: Input
;Check: SameType
test rax, 1
jne is_bool_7
is_num_8:
test rcx, 1
jne err_9
is_bool_7:
test rcx, 1
jne ok_6
err_9:
mov rdi, 1
call snek_error
ok_6:
cmp rax, [rsp - 16]
je eq_4
mov rax, 1
jmp end_5
eq_4:
mov rax, 3
end_5:
;Expr: BinOp(Equal, Bool(false), Input)
;Check: Bool
cmp rax, 1
je ok_10
cmp rax, 3
je ok_10
mov rdi, 1
call snek_error
ok_10:
cmp rax, 3
jne else_0
mov rax, 2
;Expr: Num(1)
jmp end_1
else_0:
mov rax, 4
;Expr: Num(2)
end_1:
;Expr: If(BinOp(Equal, Bool(false), Input), Num(1), Num(2)) ret
