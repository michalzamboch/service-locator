
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::types::{animal_type::AnimalTypes, animal_trait::Animal, cat::Cat, dog::Dog, unknown_animal::UnknownAnimal, creatable_trait::ICreatable};


pub fn animal_creator(animal_type: AnimalTypes) -> Rc<RefCell<dyn Animal>> {
    match animal_type {
        AnimalTypes::Cat => Rc::new(RefCell::new(Cat::new("Lilly"))),
        AnimalTypes::Dog => Rc::new(RefCell::new(Dog::new("Rex"))),
    }
}

// --------------------------------------------------

pub fn object_creator<T: Default + ICreatable>(animal_type: AnimalTypes) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(T::new("")))
}

pub fn object_creator_test<T: Default + ICreatable>(animal_type: AnimalTypes) -> Rc<RefCell<T>> {
    let x = object_creator::<T>(AnimalTypes::Cat);
    x.clone()
}

// --------------------------------------------------

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

// ------------------------------------------------------------------------

/*
trait IObjectModel<T: Default + Clone> {
    fn get(&self, path: String) -> Rc<RefCell<T>>;
}

#[derive(Clone, Debug)]
pub struct ObectModel;

impl<T: Clone + Default> IObjectModel<T> for ObectModel {
    fn get(&self, path: String) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(T::default()))
    }
}

pub fn get_type<T: Default>(animal_type: AnimalTypes) -> Rc<RefCell<T>> {
    let animals: HashMap<AnimalTypes, Rc<RefCell<T>>> = HashMap::new();
    let animal = animals.get(&animal_type);

    match animal {
        Some(success) => success.clone(),
        None => Rc::new(RefCell::new(T::default())),
    }
}
*/