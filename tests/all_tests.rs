mod infra;

success_tests! {
    test_input: { file: "input", input: "5", expected: "10" },
    test_block_set: { file: "block_set", expected: "6" },
    test_nested_let: { file: "nested_let", expected: "-6" },
}

runtime_error_tests! {
    test_invalid_input: { file: "invalid_input", input: "4", expected: "ERROR (code 1): invalid argument" },
    test_if: { file: "invalid_if", expected: "ERROR (code 1): invalid argument" },
    test_equal: { file: "invalid_equal", expected: "ERROR (code 1): invalid argument" },
    test_overflow1: { file: "mul_of", expected: "ERROR (code 2): overflow" },
    test_overflow2: { file: "mul_of2", expected: "ERROR (code 2): overflow" },
    test_invalid_arg: { file: "mul_bool", expected: "ERROR (code 1): invalid argument" },
}

static_error_tests! {

}

repl_tests! {
    test_simple_bools: { commands: ["(define x true)", "x", "false"], expected: ["true", "false"] },
}
