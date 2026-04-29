section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        fun_p:
;Function: p with 1 args
mov rax, [rsp - 8]
mov rdi, rax
push rsp
push rax
call snek_print
pop rax
pop rsp
;Return from function
jmp [rsp - 0]
        our_code_starts_here:
 ;Call: p with 1 args
;Save return address
lea rcx, [rel after_call_p_0]
mov [rsp - 8], rcx
;Evaluate and store arguments
mov rax, 134
mov [rsp - 16], rax
;Adjust RSP
sub rsp, 8
;Jump to function p
jmp fun_p
after_call_p_0:
;Returned from function
add rsp, 8
;Main expression
ret
