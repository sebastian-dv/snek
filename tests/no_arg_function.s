section .text
        extern snek_error
        extern snek_print
        global our_code_starts_here
        our_code_starts_here:
 fun_add:
;Function: add with 0 args
mov rax, 12
add rax, 2
;Return from function
ret
;Call: add with 0 args
;Save return address
lea rcx, after_call_add_0
mov [rsp - 16], rcx
;Evaluate and store arguments
;Adjust RSP
sub rsp, 16
;Jump to function add
jmp fun_add
after_call_add_0:
;Returned from function
add rsp, 16
;Main expression
ret
