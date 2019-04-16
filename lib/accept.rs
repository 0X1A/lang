use visitor::*;

pub trait Accept: Sized {
    fn accept<T>(&self, visitor: &mut T) -> T::Value
    where
        T: Visitor<Self>,
    {
        visitor.visit(self)
    }
}
