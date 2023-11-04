use super::creatable_trait::ICreatable;

use super::animal_trait::Animal;

#[derive(Debug, Default)]
pub struct Cat {
    name: String,
}

impl ICreatable for Cat {
    fn new(name: &str) -> Cat {
        Cat {
            name: name.to_owned(),
        }
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says meow", self.name);
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}
