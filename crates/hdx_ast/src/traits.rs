use crate::Atomizable;
pub trait Unit: Sized {
    type Unit: Atomizable + Default;

    fn new(value: f32, unit: Self::Unit) -> Self;
}
