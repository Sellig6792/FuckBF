use crate::ast::instructions::{InstructionTrait, InstructionType};

use crate::evaluation::Cell;

#[derive(Clone)]
pub struct Scope<T: InstructionTrait<T>> {
    memory: Vec<Cell>,
    function_memory: Vec<T>,
}

impl<T: InstructionTrait<T>> Scope<T> {
    pub fn new() -> Scope<T> {
        let mut function_memory_vec = vec![];
        for _ in 0..30000 {
            function_memory_vec.push(T::new(InstructionType::Function, Some(vec![])));
        }

        Scope {
            memory: vec![Cell::new(0); 30000],
            function_memory: match function_memory_vec.try_into() {
                Ok(function_memory) => function_memory,
                Err(_) => panic!("Could not convert Vec to Box<[T; 30000]>"),
            },
        }
    }

    pub fn get_cell(&self, index: usize) -> &Cell {
        &self.memory[index]
    }

    pub fn get_cell_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.memory[index]
    }

    pub fn get_function(&self, index: usize) -> &T {
        &self.function_memory[index]
    }

    pub fn get_function_mut(&mut self, index: usize) -> &mut T {
        &mut self.function_memory[index]
    }
}

pub struct Scopes<T>
where
    T: InstructionTrait<T>,
{
    index: usize,
    scopes: Vec<Scope<T>>,
    scope_index: usize,
}

impl<T> Scopes<T>
where
    T: InstructionTrait<T>,
{
    pub fn new() -> Scopes<T> {
        Scopes {
            index: 0,
            scopes: vec![Scope::new()],
            scope_index: 0,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_scope_index(&self) -> usize {
        self.scope_index
    }

    pub fn get_current_scope(&self) -> &Scope<T> {
        &self.scopes[self.scope_index]
    }

    pub fn get_current_scope_mut(&mut self) -> &mut Scope<T> {
        &mut self.scopes[self.scope_index]
    }

    pub fn get_scope_at(&self, index: usize) -> Option<&Scope<T>> {
        self.scopes.get(index)
    }

    pub fn get_scope_at_mut(&mut self, index: usize) -> Option<&mut Scope<T>> {
        self.scopes.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    pub fn move_right(&mut self, amount: usize) {
        // If the index exceeds the memory size, add a new cell to the memory and set the index to the new cell
        let new_index = self.index + amount;

        match self.get_cell_at(new_index) {
            Some(_) => self.index = new_index,
            None => {
                let exceed = new_index - self.get_current_scope().memory.len();
                let new_cells = vec![Cell::new(0); exceed];
                self.get_current_scope_mut().memory.extend(new_cells);
                self.index = new_index;
            }
        }
    }

    pub fn move_left(&mut self, amount: usize) {
        // If the index is less than 0, set the index to the last cell
        let sub: isize = self.index as isize - amount as isize;

        self.index = if sub < 0 {
            self.get_current_scope().memory.len() - (sub.abs() as usize - 1) - 1
        } else {
            sub as usize
        };
    }

    pub fn move_right_scope(&mut self, amount: usize) {
        // If the index exceeds the number of scopes, set the index to the last scope
        let new_scope_pointer = self.scope_index + amount;

        self.scope_index = if new_scope_pointer >= self.scopes.len() {
            self.scopes.len() - 1
        } else {
            new_scope_pointer
        };
    }

    pub fn move_left_scope(&mut self, amount: usize) {
        // If the index is less than 0, set the index to the first scope
        let sub: isize = self.scope_index as isize - amount as isize;

        self.scope_index = if sub < 0 { 0 } else { sub as usize };
    }

    pub fn get_current_cell(&self) -> &Cell {
        self.get_cell_at(self.index).unwrap()
    }

    pub fn get_current_cell_mut(&mut self) -> &mut Cell {
        self.get_cell_at_mut(self.index).unwrap()
    }

    pub fn get_cell_at(&self, index: usize) -> Option<&Cell> {
        self.get_current_scope().memory.get(index)
    }

    pub fn get_cell_at_mut(&mut self, index: usize) -> Option<&mut Cell> {
        self.get_current_scope_mut().memory.get_mut(index)
    }

    pub fn get_current_function(&self) -> &T {
        self.get_function_at(self.index).unwrap()
    }

    pub fn get_current_function_mut(&mut self) -> &mut T {
        self.get_function_at_mut(self.index).unwrap()
    }

    pub fn get_function_at(&self, index: usize) -> Option<&T> {
        self.get_current_scope().function_memory.get(index)
    }

    pub fn get_function_at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_current_scope_mut().function_memory.get_mut(index)
    }

    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
        self.scope_index += 1;
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
        self.scope_index -= 1;
    }
}
