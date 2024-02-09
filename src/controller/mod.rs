// controller/mod.rs
pub mod error;
pub mod estimate_controller;
pub mod section_controller;
use crate::Result;

pub trait Executable<T> {
    fn execute(&mut self, input: T);
}

pub struct GenericController<U> {
    use_case: U,
}

impl<U> GenericController<U> {
    pub fn new(use_case: U) -> Self {
        Self { use_case }
    }
}

impl<U, T> Executable<T> for GenericController<U>
where
    U: Executable<T>,
{
    fn execute(&mut self, input: T) {
        self.use_case.execute(input);
    }
}
