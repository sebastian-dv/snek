section .text
        extern snek_error
        global our_code_starts_here
        our_code_starts_here:
 mov rax, 4
;Expr: Num(2)
mov [rsp - 16], rax
mov rax, 6
;Expr: Num(3)
mov [rsp - 24], rax
mov rax, 0
;Expr: Num(0)
mov [rsp - 32], rax
mov rax, 0
;Expr: Num(0)
mov [rsp - 40], rax
mov rax, 0
;Expr: Num(0)
mov [rsp - 48], rax
loop_start_0:
;Less
mov rax, [rsp - 16]
;Expr: Id("a")
;Check: Num
test rax, 1
je ok_4
mov rdi, 1
call snek_error
ok_4:
mov [rsp - 56], rax
mov rax, [rsp - 40]
;Expr: Id("i")
;Check: Num
test rax, 1
je ok_5
mov rdi, 1
call snek_error
ok_5:
cmp rax, [rsp - 56]
jl less_6
mov rax, 1
jmp end_7
less_6:
mov rax, 3
end_7:
;Expr: BinOp(Less, Id("a"), Id("i"))
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
mov rax, 0
;Expr: Num(0)
mov [rsp - 48], rax
;Expr: Set("j", Num(0))
loop_start_9:
;Less
mov rax, [rsp - 24]
;Expr: Id("b")
;Check: Num
test rax, 1
je ok_13
mov rdi, 1
call snek_error
ok_13:
mov [rsp - 72], rax
mov rax, [rsp - 48]
;Expr: Id("j")
;Check: Num
test rax, 1
je ok_14
mov rdi, 1
call snek_error
ok_14:
cmp rax, [rsp - 72]
jl less_15
mov rax, 1
jmp end_16
less_15:
mov rax, 3
end_16:
;Expr: BinOp(Less, Id("b"), Id("j"))
;Check: Bool
cmp rax, 1
je ok_17
cmp rax, 3
je ok_17
mov rdi, 1
call snek_error
ok_17:
cmp rax, 3
jne else_11
mov rax, [rsp - 32]
;Expr: Id("c")
;Sub1
sub rax, 2
;Expr: UnOp(Sub1, Id("c"))
mov [rsp - 32], rax
;Expr: Set("c", UnOp(Sub1, Id("c")))
mov rax, [rsp - 48]
;Expr: Id("j")
;Add1
add rax, 2
;Expr: UnOp(Add1, Id("j"))
mov [rsp - 48], rax
;Expr: Set("j", UnOp(Add1, Id("j")))
;Expr: Block([Set("c", UnOp(Sub1, Id("c"))), Set("j", UnOp(Add1, Id("j")))])
jmp end_12
else_11:
mov rax, [rsp - 32]
;Expr: Id("c")
jmp loop_end_10
;Expr: Break(Id("c"))
end_12:
;Expr: If(BinOp(Less, Id("b"), Id("j")), Block([Set("c", UnOp(Sub1, Id("c"))), Set("j", UnOp(Add1, Id("j")))]), Break(Id("c")))
jmp loop_start_9
loop_start_9:
;Expr: Loop(If(BinOp(Less, Id("b"), Id("j")), Block([Set("c", UnOp(Sub1, Id("c"))), Set("j", UnOp(Add1, Id("j")))]), Break(Id("c"))))
mov rax, [rsp - 40]
;Expr: Id("i")
;Add1
add rax, 2
;Expr: UnOp(Add1, Id("i"))
mov [rsp - 40], rax
;Expr: Set("i", UnOp(Add1, Id("i")))
;Expr: Block([Set("j", Num(0)), Loop(If(BinOp(Less, Id("b"), Id("j")), Block([Set("c", UnOp(Sub1, Id("c"))), Set("j", UnOp(Add1, Id("j")))]), Break(Id("c")))), Set("i", UnOp(Add1, Id("i")))])
jmp end_3
else_2:
mov rax, [rsp - 32]
;Expr: Id("c")
jmp loop_end_1
;Expr: Break(Id("c"))
end_3:
;Expr: If(BinOp(Less, Id("a"), Id("i")), Block([Set("j", Num(0)), Loop(If(BinOp(Less, Id("b"), Id("j")), Block([Set("c", UnOp(Sub1, Id("c"))), Set("j", UnOp(Add1, Id("j")))]), Break(Id("c")))), Set("i", UnOp(Add1, Id("i")))]), Break(Id("c")))
jmp loop_start_0
loop_start_0:
;Expr: Loop(If(BinOp(Less, Id("a"), Id("i")), Block([Set("j", Num(0)), Loop(If(BinOp(Less, Id("b"), Id("j")), Block([Set("c", UnOp(Sub1, Id("c"))), Set("j", UnOp(Add1, Id("j")))]), Break(Id("c")))), Set("i", UnOp(Add1, Id("i")))]), Break(Id("c"))))
;Expr: Let([("a", Num(2)), ("b", Num(3)), ("c", Num(0)), ("i", Num(0)), ("j", Num(0))], Loop(If(BinOp(Less, Id("a"), Id("i")), Block([Set("j", Num(0)), Loop(If(BinOp(Less, Id("b"), Id("j")), Block([Set("c", UnOp(Sub1, Id("c"))), Set("j", UnOp(Add1, Id("j")))]), Break(Id("c")))), Set("i", UnOp(Add1, Id("i")))]), Break(Id("c"))))) ret
