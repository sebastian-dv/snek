section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        fun_sum:
;Function: sum with 3 args
mov rax, [rsp - 24]
test rax, 1
mov rax, 3
mov [rsp - 32], 1
cmove rax, [rsp - 32]
;Check: Bool
cmp rax, 1
je ok_2
cmp rax, 3
je ok_2
mov rdi, 1
call snek_error
ok_2:
cmp rax, 3
jne else_0
;Equal
mov rax, 3
mov rcx, rax
mov [rsp - 40], rax
mov rax, [rsp - 24]
;Check: SameType
test rax, 1
jne is_bool_10
is_num_11:
test rcx, 1
jne err_12
jmp ok_9
is_bool_10:
test rcx, 1
jne ok_9
err_12:
mov rdi, 1
call snek_error
ok_9:
cmp rax, [rsp - 40]
je eq_7
mov rax, 1
jmp end_8
eq_7:
mov rax, 3
end_8:
;Check: Bool
cmp rax, 1
je ok_13
cmp rax, 3
je ok_13
mov rdi, 1
call snek_error
ok_13:
cmp rax, 3
jne else_3
mov rax, 2
jmp end_4
else_3:
mov rax, -2
end_4:
jmp end_1
else_0:
mov rax, [rsp - 24]
end_1:
mov [rsp - 32], rax
mov rax, 2
mov [rsp - 40], rax
mov rax, [rsp - 8]
mov [rsp - 48], rax
loop_start_14:
;Greater
mov rax, 0
;Check: Num
test rax, 1
je ok_20
mov rdi, 1
call snek_error
ok_20:
mov [rsp - 56], rax
mov rax, [rsp - 32]
;Check: Num
test rax, 1
je ok_21
mov rdi, 1
call snek_error
ok_21:
cmp rax, [rsp - 56]
jg greater_22
mov rax, 1
jmp end_23
greater_22:
mov rax, 3
end_23:
;Check: Bool
cmp rax, 1
je ok_24
cmp rax, 3
je ok_24
mov rdi, 1
call snek_error
ok_24:
cmp rax, 3
jne else_18
;GreaterEqual
mov rax, [rsp - 16]
;Check: Num
test rax, 1
je ok_25
mov rdi, 1
call snek_error
ok_25:
mov [rsp - 64], rax
mov rax, [rsp - 48]
;Check: Num
test rax, 1
je ok_26
mov rdi, 1
call snek_error
ok_26:
cmp rax, [rsp - 64]
jge greater_equal_27
mov rax, 1
jmp end_28
greater_equal_27:
mov rax, 3
end_28:
jmp end_19
else_18:
;LessEqual
mov rax, [rsp - 16]
;Check: Num
test rax, 1
je ok_29
mov rdi, 1
call snek_error
ok_29:
mov [rsp - 72], rax
mov rax, [rsp - 48]
;Check: Num
test rax, 1
je ok_30
mov rdi, 1
call snek_error
ok_30:
cmp rax, [rsp - 72]
jle less_equal_31
mov rax, 1
jmp end_32
less_equal_31:
mov rax, 3
end_32:
end_19:
;Check: Bool
cmp rax, 1
je ok_33
cmp rax, 3
je ok_33
mov rdi, 1
call snek_error
ok_33:
cmp rax, 3
jne else_16
mov rax, [rsp - 40]
jmp loop_end_15
jmp end_17
else_16:
;Plus
mov rax, [rsp - 40]
;Check: Num
test rax, 1
je ok_34
mov rdi, 1
call snek_error
ok_34:
mov [rsp - 72], rax
mov rax, [rsp - 48]
;Check: Num
test rax, 1
je ok_35
mov rdi, 1
call snek_error
ok_35:
add rax, [rsp - 72]
;Check: Overflow
jo err_37
jmp ok_36
err_37:
mov rdi, 2
call snek_error
ok_36:
mov [rsp - 40], rax
;Plus
mov rax, [rsp - 48]
;Check: Num
test rax, 1
je ok_38
mov rdi, 1
call snek_error
ok_38:
mov [rsp - 80], rax
mov rax, [rsp - 32]
;Check: Num
test rax, 1
je ok_39
mov rdi, 1
call snek_error
ok_39:
add rax, [rsp - 80]
;Check: Overflow
jo err_41
jmp ok_40
err_41:
mov rdi, 2
call snek_error
ok_40:
mov [rsp - 48], rax
end_17:
jmp loop_start_14
loop_end_15:
;Return from function
jmp [rsp - 0]
        our_code_starts_here:
 mov rax, 19998
mov rdi, rax
push rsp
push rax
call snek_print
pop rax
pop rsp
;Call: sum with 3 args
;Save return address
lea rcx, [rel after_call_sum_42]
mov [rsp - 16], rcx
;Evaluate and store arguments
mov rax, 6
mov [rsp - 24], rax
mov rax, 20
mov [rsp - 32], rax
mov rax, 2
mov [rsp - 40], rax
;Adjust RSP
sub rsp, 16
;Jump to function sum
jmp fun_sum
after_call_sum_42:
;Returned from function
add rsp, 16
mov rdi, rax
push rsp
push rax
call snek_print
pop rax
pop rsp
;Call: sum with 3 args
;Save return address
lea rcx, [rel after_call_sum_43]
mov [rsp - 24], rcx
;Evaluate and store arguments
mov rax, 6
mov [rsp - 32], rax
mov rax, 20
mov [rsp - 40], rax
mov rax, 2
mov [rsp - 48], rax
;Adjust RSP
sub rsp, 24
;Jump to function sum
jmp fun_sum
after_call_sum_43:
;Returned from function
add rsp, 24
mov rdi, rax
push rsp
push rax
call snek_print
pop rax
pop rsp
;Call: sum with 3 args
;Save return address
lea rcx, [rel after_call_sum_44]
mov [rsp - 32], rcx
;Evaluate and store arguments
mov rax, 20
mov [rsp - 40], rax
mov rax, 6
mov [rsp - 48], rax
mov rax, 1
mov [rsp - 56], rax
;Adjust RSP
sub rsp, 32
;Jump to function sum
jmp fun_sum
after_call_sum_44:
;Returned from function
add rsp, 32
mov rdi, rax
push rsp
push rax
call snek_print
pop rax
pop rsp
;Main expression
ret
