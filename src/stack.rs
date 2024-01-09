use std::fmt::Debug;
use std::fmt;

pub struct Stack<T> {
    pointer: usize,
    capacity: usize,
    data: Vec<T>
}

#[derive(Debug, PartialEq)]
pub enum StackError {
    Overflow,
    Underflow,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack overflow error"),
            StackError::Underflow => write!(f, "Stack underflow error"),
        }
    }
}

impl<T: Default+ Clone + Debug> fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stack {{ capacity: {}, pointer: {}, data: {:?} }}",
            self.capacity,
            self.pointer,
            self.data.iter().take(self.pointer).cloned().collect::<Vec<_>>(),
        )
    }
}

impl<T: Default+ Clone + Debug> Stack<T>  {
    pub fn new(size: usize) -> Stack<T> {
        Stack {
            pointer: 0,
            capacity: size,
            data: vec![T::default(); size]
        }
    }

    pub fn size(&self) -> usize {
        self.pointer
    }
    pub fn is_empty(&self) -> bool {
        self.pointer == 0
    }

    pub fn top(&self) -> Option<T>{
        if self.is_empty() {
            None
        }else{
            Some(self.data[self.pointer-1].clone())
        }
    }

    pub fn push(&mut self, value : T) -> Result<(), StackError> {
        if self.pointer >= self.capacity {
            return Err(StackError::Overflow);
        }

        self.data[self.pointer] = value;
        self.pointer += 1;
        Ok(())
    }  

    pub fn pop(&mut self) -> Result<T, StackError>{
        if self.pointer == 0 {
            return Err(StackError::Underflow);
        }

        let value = self.data[self.pointer-1].clone();
        self.pointer -= 1;
        Ok(value)
    } 
}