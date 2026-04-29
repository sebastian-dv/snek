pub static SNEK_TRUE: i64 = 3;
pub static SNEK_FALSE: i64 = 1;

pub fn bool_to_snek(b: bool) -> i64 {
    if b {
        3
    } else {
        1
    }
}

pub fn num_to_snek(n: &i64) -> i64 {
    n << 1
}

pub fn from_snek(n: &i64) -> String {
    if *n == 3 {
        "true".to_string()
    } else if *n == 1 {
        "false".to_string()
    } else if *n == 5 {
        "Runtime Error".to_string()
    } else {
        let num = *n / 2;
        (num).to_string()
    }
}

