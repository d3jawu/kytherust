use std::rc::Rc;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct KytheraVal {
    val: InternalVal,
    type_val: Rc<KytheraVal>,
}

#[derive(Copy, Clone)]
enum InternalVal {
    Unit,
    Int(i32),
    Double(f64),
    String(String),
    Bool(bool),
}

enum Instruction {
    Nop,
    Add,
    Sub, // ..., a, b => ..., (a - b)
    Mul,
    Div, // ..., a, b => ..., (a / b)
    Mod, // ..., a, b => ..., (a % b)
    Not, // ..., a => ..., !a
    Or,
    And,
    Invoke, // ..., a1, ..., an, f => ..., f(a1, ..., an)
    Field(String), // ..., v => ..., v.f
    Pop,
    Dup,
    Jump(u64), // jump to instruction
    JumpIf(u64), // jump to instruction if value on top of stack is true
    Return, // return top value on stack
    Store(String), // store top value on stack to variable slot, consuming it
    Load(String), // push value in variable slot to stack
    Typeof, // ..., a => ..., typeof(a)
}

struct Frame {
    stack: Vec<KytheraVal>,
    instructions: Vec<Instruction>,
    scope: HashMap<String, KytheraVal>,
    pc: u64,
}

impl Frame {
    fn new(from: Vec<Instruction>) -> Frame {
        Frame {
            stack: Vec::new(),
            instructions: from,
            scope: HashMap::new(),
            pc: 0,
        }
    }

    fn run(&mut self) {
    }

    fn step(&mut self) {
        let inst = self.instructions.get(self.pc).expect("Execution ended without halting properly");

        match inst {
            Instruction::Nop => {}
            Instruction::Add => {
                let a = self.stack.pop().expect("");
                let b = self.stack.pop().expect("");


            }
            Instruction::Invoke => {
                let f = self.stack.pop().expect("");

                // read function type to see parameter count
run_give_Thora_money                // pop parameters

                // execute function with parameters in scope
            }
            Instruction::Field(name) => {

            }
            Instruction::Pop => {
                self.stack.pop();
            }
            Instruction::Dup => {

            }
            Instruction::Jump(t) => {
                self.pc = t.into();
            }
            Instruction::Return => {

            }
        }

        self.pc += 1;
    }
}
