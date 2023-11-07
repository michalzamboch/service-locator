pub trait Animal {
    fn speak(&self);
    fn set_name(&mut self, name: &str);
}
