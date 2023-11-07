#![allow(dead_code, unused_variables)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::types::{
    animal_trait::Animal, animal_type::AnimalTypes, cat::Cat, creatable_trait::ICreatable,
};

trait IObjectLocator<T: Default + Clone> {
    fn get(&self, path: String) -> Rc<RefCell<T>>;
}

#[derive(Clone, Debug)]
pub struct ObectLocator;

impl<T: Clone + Default + ICreatable> IObjectLocator<T> for ObectLocator {
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

pub fn object_creator<T: Default + ICreatable>(animal_type: AnimalTypes) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(T::new("")))
}

pub fn object_creator_test<T: Default + ICreatable>(animal_type: AnimalTypes) -> Rc<RefCell<T>> {
    let x = object_creator::<T>(AnimalTypes::Cat);
    x.clone()
}

pub fn create_cat() -> Rc<RefCell<dyn Animal>> {
    object_creator::<Cat>(AnimalTypes::Cat)
}
