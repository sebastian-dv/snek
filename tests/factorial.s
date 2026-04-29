section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        fun_fact:
;Function: fact with 1 args
mov rax, 2
mov [rsp - 16], rax
mov rax, 2
mov [rsp - 24], rax
loop_start_0:
;Greater
mov rax, [rsp - 8]
;Check: Num
test rax, 1
je ok_4
mov rdi, 1
call snek_error
ok_4:
mov [rsp - 32], rax
mov rax, [rsp - 16]
;Check: Num
test rax, 1
je ok_5
mov rdi, 1
call snek_error
ok_5:
cmp rax, [rsp - 32]
jg greater_6
mov rax, 1
jmp end_7
greater_6:
mov rax, 3
end_7:
;Check: Bool
cmp rax, 1
je ok_8
cmp rax, 3
je ok_8
mov rdi, 1
call snek_error
ok_8:
cmp rax, 3
jne else_2
mov rax, [rsp - 24]
jmp loop_end_1
jmp end_3
else_2:
;Times
mov rax, [rsp - 24]
;Check: Num
test rax, 1
je ok_9
mov rdi, 1
call snek_error
ok_9:
sar rax, 1
mov [rsp - 48], rax
mov rax, [rsp - 16]
;Check: Num
test rax, 1
je ok_10
mov rdi, 1
call snek_error
ok_10:
imul rax, [rsp - 48]
;Check: Overflow
jo err_12
jmp ok_11
err_12:
mov rdi, 2
call snek_error
ok_11:
mov [rsp - 24], rax
;Plus
mov rax, [rsp - 16]
;Check: Num
test rax, 1
je ok_13
mov rdi, 1
call snek_error
ok_13:
mov [rsp - 56], rax
mov rax, 2
;Check: Num
test rax, 1
je ok_14
mov rdi, 1
call snek_error
ok_14:
add rax, [rsp - 56]
;Check: Overflow
jo err_16
jmp ok_15
err_16:
mov rdi, 2
call snek_error
ok_15:
mov [rsp - 16], rax
end_3:
jmp loop_start_0
loop_end_1:
;Return from function
jmp [rsp - 0]
        our_code_starts_here:
 ;Call: fact with 1 args
;Save return address
lea rcx, [rel after_call_fact_17]
mov [rsp - 8], rcx
;Evaluate and store arguments
mov rax, 8
mov [rsp - 16], rax
;Adjust RSP
sub rsp, 8
;Jump to function fact
jmp fun_fact
after_call_fact_17:
;Returned from function
add rsp, 8
;Main expression
ret
