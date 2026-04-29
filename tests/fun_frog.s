section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        fun_frog:
;Function: frog with 1 args
loop_start_0:
;Equal
mov rax, 0
mov rcx, rax
mov [rsp - 16], rax
mov rax, [rsp - 8]
;Check: SameType
test rax, 1
jne is_bool_9
is_num_10:
test rcx, 1
jne err_11
jmp ok_8
is_bool_9:
test rcx, 1
jne ok_8
err_11:
mov rdi, 1
call snek_error
ok_8:
cmp rax, [rsp - 16]
je eq_6
mov rax, 1
jmp end_7
eq_6:
mov rax, 3
end_7:
;Check: Bool
cmp rax, 1
je ok_12
cmp rax, 3
je ok_12
mov rdi, 1
call snek_error
ok_12:
cmp rax, 3
jne else_2
mov rax, 3
jmp loop_end_1
jmp end_3
else_2:
mov rax, [rsp - 8]
sub rax, 2
mov [rsp - 8], rax
end_3:
loop_start_13:
;Equal
mov rax, 0
mov rcx, rax
mov [rsp - 24], rax
mov rax, [rsp - 8]
;Check: SameType
test rax, 1
jne is_bool_22
is_num_23:
test rcx, 1
jne err_24
jmp ok_21
is_bool_22:
test rcx, 1
jne ok_21
err_24:
mov rdi, 1
call snek_error
ok_21:
cmp rax, [rsp - 24]
je eq_19
mov rax, 1
jmp end_20
eq_19:
mov rax, 3
end_20:
;Check: Bool
cmp rax, 1
je ok_25
cmp rax, 3
je ok_25
mov rdi, 1
call snek_error
ok_25:
cmp rax, 3
jne else_15
mov rax, 3
jmp loop_end_14
jmp end_16
else_15:
mov rax, [rsp - 8]
mov rdi, rax
push rsp
push rax
call snek_print
pop rax
pop rsp
jmp loop_end_14
sub rax, 2
mov [rsp - 8], rax
end_16:
jmp loop_start_13
loop_end_14:
jmp loop_start_0
loop_end_1:
;Return from function
jmp [rsp - 0]
        our_code_starts_here:
 ;Call: frog with 1 args
;Save return address
lea rcx, [rel after_call_frog_26]
mov [rsp - 8], rcx
;Evaluate and store arguments
mov rax, rdi
;Check: Cast to Num
test rax, 1
je cast_ok_27
mov rdi, 3
call snek_error
cast_ok_27:
mov [rsp - 16], rax
;Adjust RSP
sub rsp, 8
;Jump to function frog
jmp fun_frog
after_call_frog_26:
;Returned from function
add rsp, 8
;Main expression
ret
