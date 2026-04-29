# snek

This is a compiler written in Rust for the custom programming
language snek. This was the course project for a fun compiler 
class I took. I got the chance to learn a little more about compilers
and get some hands on experience with writing one.

A snek program will always evaluate to an integer, boolean, or
end with an error. Supports ahead-of-time, just-in-time, and REPL 
execution modes with optional static typechecking.

## Pipeline

1. Parse snek program with Rust's sexp s-expression parser
2. Keep track of function and variable definitions
3. Convert to custom Expr data structure
4. Compile Expr to Instruction data structure
5. Convert Instruction to x86-64 assembly
    - String format for AOT compile
    - Dynasm for JIT or REPL compile

## Compile

```
# Add -t to any for typechecking
# AOT (Ahead-of-time)
cargo run -- -c tests/test1.snek tests/test1.s
# JIT (Just-in-time) 
cargo run -- -e tests/test1.snek <optionalArg>
# AOT and JIT
cargo run -- -g tests/test1.snek tests/test1.s <optionalArg>
# REPL (Read-eval-print loop)
cargo run -- -i
```

## Syntax

```
<prog> := <defn>* <expr>
<defn> := (fun (<name> <name>*) <expr>)
<expr> :=
  | <number>
  | true
  | false
  | input
  | <identifier>
  | (let (<binding>+) <expr>)
  | (<op1> <expr>)
  | (<op2> <expr> <expr>)
  | (set! <name> <expr>)
  | (if <expr> <expr> <expr>)
  | (block <expr>+)
  | (loop <expr>)
  | (break <expr>)
  | (<name> <expr>*)

<op1> := add1 | sub1 | isnum | isbool | print
<op2> := + | - | * | < | > | >= | <= | =

<binding> := (<identifier> <expr>)
```

## Example Programs

```
(fun (fact n)
  (let
    ((i 1) (acc 1))
    (loop
      (if (> i n)
        (break acc)
        (block
          (set! acc (* acc i))
          (set! i (+ i 1))
        )
      )
    )
  )
)
(fact input)
```

```
(fun (isodd n)
  (if (< n 0)
      (isodd (- 0 n))
      (if (= n 0)
          false
          (iseven (sub1 n))
      )
  )
)

(fun (iseven n)
  (if (= n 0)
      true
      (isodd (sub1 n))
  )
)

(block
  (print input)
  (print (iseven input))
)

```

## To add new tests

- Create new file `[test_name].snek` in `tests/`

- Add the name of the file to `tests/all_tests.rs` under either the success test
or failure test section along with the expected output

- Run the command `cargo test -- --test-threads 1`
