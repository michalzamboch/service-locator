use super::creatable_trait::ICreatable;

use super::animal_trait::Animal;

#[derive(Debug, Default)]
pub struct UnknownAnimal {
    name: String,
}

impl ICreatable for UnknownAnimal {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl Animal for UnknownAnimal {
    fn speak(&self) {
        println!("UNKNOWN ANIMAL {} says nothing", self.name);
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}
