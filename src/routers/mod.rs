pub mod animal_route;
pub mod user_route;

pub mod prelude {
    pub use super::animal_route::RouterAnimal;
    pub use super::user_route::RouterUser;
}