#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum AnimalTypes {
    Cat,
    #[default]
    Dog,
}
