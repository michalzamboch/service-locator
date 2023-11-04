use super::creatable_trait::ICreatable;

use super::animal_trait::Animal;

#[derive(Debug, Default)]
pub struct Dog {
    name: String,
}

impl ICreatable for Dog {
    fn new(name: &str) -> Dog {
        Dog {
            name: name.to_owned(),
        }
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says bow bow", self.name);
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}
