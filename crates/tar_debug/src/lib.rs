pub mod trace;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use tar_debug_macros::{ session, trace };
    pub use super::trace::{Trace, TraceType, Session};
}
