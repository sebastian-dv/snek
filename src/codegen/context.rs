#[derive(Clone)]
pub struct Context {
    pub si: i32,
    pub define_env: im::HashMap<String, i64>,
    pub fun_env: im::HashMap<String, i32>,
    pub stack_env: im::HashMap<String, i32>,
    pub break_label: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            si: 1,
            stack_env: im::HashMap::new(),
            define_env: im::HashMap::new(),
            fun_env: im::HashMap::new(),
            break_label: None,
        }
    }

    pub fn increment_si_by(&mut self, n: i32) {
        self.si += n;
    }

    pub fn insert_define(&mut self, name: String, ptr: i64) {
        self.define_env = self.define_env.update(name, ptr);
    }

    pub fn with_fun_env(&mut self, fun_env: im::HashMap<String, i32>) {
        self.fun_env = fun_env;
    }

    pub fn insert_fun(&mut self, name: String, arg_count: i32) {
        self.fun_env = self.fun_env.update(name, arg_count);
    }

    pub fn insert_stack(&mut self, name: String, offset: i32) {
        self.stack_env = self.stack_env.update(name, offset);
    }

    pub fn clear_stack(&mut self) {
        self.stack_env = im::HashMap::new();
    }

    pub fn with_break_label(&mut self, label: String) {
        self.break_label = Some(label);
    }

    pub fn clear_break_label(&mut self) {
        self.break_label = None;
    }

    pub fn curr_offset(&self) -> i32 {
        self.si * 8
    }
}
