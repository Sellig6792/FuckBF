use crate::ast::instructions::Instruction;

#[derive(Debug)]
pub struct Scope {
    pub memory: Box<[u8; 30000]>,
    pub function_memory: Box<[Instruction; 30000]>,
}

impl Scope {
    pub fn new() -> Scope {
        let mut function_memory_vec = Vec::new();
        for _ in 0..30000 {
            function_memory_vec.push(Instruction::default());
        }

        Scope {
            memory: Box::new([0; 30000]),
            function_memory: function_memory_vec.into_boxed_slice().try_into().unwrap(),
        }
    }
}

impl Clone for Scope {
    fn clone(&self) -> Scope {
        Scope {
            memory: self.memory.clone(),
            function_memory: self.function_memory.clone(),
        }
    }
}
