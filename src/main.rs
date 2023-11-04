use holder::AnimalHolder;
use types::animal_type::AnimalTypes;

mod locator;
mod holder;
mod types;

fn main() {
    let mut animal_holder = AnimalHolder::new();
    animal_holder.insert(AnimalTypes::Cat);

    {
        let cat = animal_holder.get(AnimalTypes::Cat);
        cat.borrow().speak();
    }

    {
        let cat = animal_holder.get(AnimalTypes::Cat);
        cat.borrow_mut().set_name("Nala");
    }

    {
        let cat = animal_holder.get(AnimalTypes::Cat);
        cat.borrow().speak();
    }
    
    let dog = animal_holder.get(AnimalTypes::Dog);
    dog.borrow().speak();
}
