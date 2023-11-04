use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::types::{
    animal_trait::Animal, animal_type::AnimalTypes, cat::Cat, creatable_trait::ICreatable,
    dog::Dog, unknown_animal::UnknownAnimal,
};

pub type AnimalRef = Rc<RefCell<dyn Animal>>;

pub struct AnimalHolder {
    animals: HashMap<AnimalTypes, AnimalRef>,
}

impl AnimalHolder {
    pub fn new() -> Self {
        Self {
            animals: HashMap::default(),
        }
    }

    pub fn insert(&mut self, animal_type: AnimalTypes) {
        self.animals
            .insert(animal_type, animal_creator(animal_type));
    }

    pub fn get(&self, animal_type: AnimalTypes) -> AnimalRef {
        let animal = self.animals.get(&animal_type);

        match animal {
            Some(success) => success.clone(),
            None => Rc::new(RefCell::new(UnknownAnimal::new("***"))),
        }
    }
}

pub fn animal_creator(animal_type: AnimalTypes) -> Rc<RefCell<dyn Animal>> {
    match animal_type {
        AnimalTypes::Cat => Rc::new(RefCell::new(Cat::new("Lilly"))),
        AnimalTypes::Dog => Rc::new(RefCell::new(Dog::new("Rex"))),
    }
}
