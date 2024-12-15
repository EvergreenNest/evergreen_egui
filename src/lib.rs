//! An alternative integration of the immediate mode GUI crate `egui` with the
//! Bevy game engine.
//!
//! # Why not use `bevy_egui`?
//!
//! `bevy_egui`'s current way of integrating `egui` works well enough for simple
//! cases, but when you want to create more complex UIs, you'll quickly run into
//! limitations:
//!
//! - You're required to write your UI code in a single exclusive system that
//!   takes a `&mut World`. This makes it difficult to split your UI code into
//!   smaller, reusable components.

#![warn(missing_docs)]

// pub mod command;
pub mod container;
pub mod ctx;
pub mod root;
pub mod ui;
pub mod widget;

pub mod prelude {
    //! Commonly used traits and types.

    // pub use crate::command::*;
    pub use crate::container::*;
    pub use crate::ctx::*;
    pub use crate::root::*;
    pub use crate::ui::*;
    pub use crate::widget::*;
}
