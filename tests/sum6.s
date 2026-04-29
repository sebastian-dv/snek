section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        our_code_starts_here:
 fun_sum6:
;Function: sum6 with 6 args
;Plus
mov rax, [rsp - 8]
;Check: Num
test rax, 1
je ok_0
mov rdi, 1
call snek_error
ok_0:
mov [rsp - 56], rax
;Plus
mov rax, [rsp - 16]
;Check: Num
test rax, 1
je ok_2
mov rdi, 1
call snek_error
ok_2:
mov [rsp - 64], rax
;Plus
mov rax, [rsp - 24]
;Check: Num
test rax, 1
je ok_4
mov rdi, 1
call snek_error
ok_4:
mov [rsp - 72], rax
;Plus
mov rax, [rsp - 32]
;Check: Num
test rax, 1
je ok_6
mov rdi, 1
call snek_error
ok_6:
mov [rsp - 80], rax
;Plus
mov rax, [rsp - 40]
;Check: Num
test rax, 1
je ok_8
mov rdi, 1
call snek_error
ok_8:
mov [rsp - 88], rax
mov rax, [rsp - 48]
;Check: Num
test rax, 1
je ok_9
mov rdi, 1
call snek_error
ok_9:
add rax, [rsp - 88]
;Check: Overflow
jo err_11
jmp ok_10
err_11:
mov rdi, 2
call snek_error
ok_10:
;Check: Num
test rax, 1
je ok_7
mov rdi, 1
call snek_error
ok_7:
add rax, [rsp - 80]
;Check: Overflow
jo err_13
jmp ok_12
err_13:
mov rdi, 2
call snek_error
ok_12:
;Check: Num
test rax, 1
je ok_5
mov rdi, 1
call snek_error
ok_5:
add rax, [rsp - 72]
;Check: Overflow
jo err_15
jmp ok_14
err_15:
mov rdi, 2
call snek_error
ok_14:
;Check: Num
test rax, 1
je ok_3
mov rdi, 1
call snek_error
ok_3:
add rax, [rsp - 64]
;Check: Overflow
jo err_17
jmp ok_16
err_17:
mov rdi, 2
call snek_error
ok_16:
;Check: Num
test rax, 1
je ok_1
mov rdi, 1
call snek_error
ok_1:
add rax, [rsp - 56]
;Check: Overflow
jo err_19
jmp ok_18
err_19:
mov rdi, 2
call snek_error
ok_18:
;Return from function
jmp [rsp - 0]
;Call: sum6 with 6 args
;Save return address
lea rcx, [rel after_call_sum6_20]
mov [rsp - 8], rcx
;Evaluate and store arguments
mov rax, 2
mov [rsp - 16], rax
mov rax, 4
mov [rsp - 24], rax
mov rax, 6
mov [rsp - 32], rax
mov rax, 8
mov [rsp - 40], rax
mov rax, 10
mov [rsp - 48], rax
mov rax, 12
mov [rsp - 56], rax
;Adjust RSP
sub rsp, 8
;Jump to function sum6
jmp fun_sum6
after_call_sum6_20:
;Returned from function
add rsp, 8
;Main expression
ret
