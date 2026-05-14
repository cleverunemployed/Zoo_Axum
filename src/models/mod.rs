
pub mod animals;
pub mod users;

pub mod prelude {
    pub use super::animals::{Animal};
    pub use super::users::{User, UserAnimals};
}