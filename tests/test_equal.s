section .text
        extern snek_error
        global our_code_starts_here
        our_code_starts_here:
 ;Equal
mov rax, 10
;Expr: Num(5)
mov rcx, rax
mov [rsp - 16], rax
mov rax, 3
;Expr: Bool(true)
;Check: SameType
test rax, 1
jne is_bool_5
is_num_6:
test rcx, 1
jne err_7
is_bool_5:
test rcx, 1
jne ok_4
err_7:
mov rdi, 1
call snek_error
ok_4:
cmp rax, [rsp - 16]
je eq_2
mov rax, 1
jmp end_3
eq_2:
mov rax, 3
end_3:
;Expr: BinOp(Equal, Num(5), Bool(true)) ret
