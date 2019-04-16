pub trait Visitor<T> {
    type Value;
    fn visit(&mut self, expr: &T) -> Self::Value;
}
