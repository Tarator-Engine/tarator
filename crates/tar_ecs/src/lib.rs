pub mod archetype;
pub mod bundle;
pub mod callback;
pub mod component;
pub mod entity;
pub mod store;
pub mod world;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::callback::Callback;
    pub use super::component::Component;
    pub use super::entity::Entity;
    pub use super::world::World;
    pub use tar_ecs_macros::{ Callback, Component };
    pub use super::init_ecs;
}


pub fn init_ecs() {
    unsafe {
        component::Components::new();
        bundle::Bundles::new();
    }
}
