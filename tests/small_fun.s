section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        our_code_starts_here:
 fun_f:
;Function: f with 1 args
;Plus
mov rax, [rsp - 16]
;Check: Num
test rax, 1
je ok_0
mov rdi, 1
call snek_error
ok_0:
mov [rsp - 32], rax
mov rax, 2
;Check: Num
test rax, 1
je ok_1
mov rdi, 1
call snek_error
ok_1:
add rax, [rsp - 32]
;Check: Overflow
jo err_3
jmp ok_2
err_3:
mov rdi, 2
call snek_error
ok_2:
;Return from function
jmp [rsp - 0]
;Call: f with 1 args
;Save return address
lea rcx, [rel after_call_f_4]
mov [rsp - 16], rcx
;Evaluate and store arguments
mov rax, 20
mov [rsp - 32], rax
;Adjust RSP
sub rsp, 16
;Jump to function f
jmp fun_f
after_call_f_4:
;Returned from function
add rsp, 16
;Main expression
ret
