pub struct FrameNode {
    pub children: Vec<FrameNode>,
}

impl FrameNode {
    pub fn new() -> FrameNode {
        FrameNode {
            children: Vec::new(),
        }
    }
}

pub struct CallTree {
    pub root: FrameNode,
    pub static_scope: HashMap<Symbol, Value>,
    pub static_functions: HashMap<Symbol, Box<StaticFn>>,
    pub language_functions: HashMap<Symbol, Box<LanguageFn>>,
}

impl CallTree {

    pub fn new() -> CallTree {
        let mut language_functions_map: HashMap<Symbol, Box<LanguageFn>> = HashMap::new();
        language_functions::init(&mut language_functions_map);

        let mut static_functions_map: HashMap<Symbol, Box<StaticFn>> = HashMap::new();
        static_functions::init(&mut static_functions_map);

        let mut static_scope_map: HashMap<Symbol, Value> = HashMap::new();
        static_scope::init(&mut static_scope_map);

        CallTree {
            stack: FrameNode::new(),
            static_scope: HashMap::new(),
            static_functions: static_functions_map,
            language_functions: language_functions_map,
        }
    }

    pub fn push_value(&mut self, symbol: Symbol, value: Value) {
        if self.stack.len() > 0 {
            let i = self.stack.len() - 1;
            self.stack[i].scope.insert(symbol, value);
        }
    }

    pub fn get_value(&self, symbol: &Symbol) -> Value {

        if let Some(value) = self.static_scope.get(symbol) {

            value.clone()

        } else {

            let mut i = self.stack.len() - 1;
            loop {
                if let Some(val) = self.stack[i].scope.get(symbol) {
                    return val.clone()
                }
                else if i == 0 {
                    return Value::Null
                }
            }
        }
    }

    pub fn push_frame(&mut self) {
        self.stack.push(Frame::new());
    }

    pub fn pop_return(&mut self) -> Value {
        if let Some(frame) = self.stack.pop() {
            frame.returns
        } else { Value::Null }
    }

    pub fn top(&mut self) -> &mut Frame {
        let len = self.stack.len();
        &mut self.stack[len - 1]
    }

}
