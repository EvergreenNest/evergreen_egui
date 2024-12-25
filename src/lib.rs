//! An improved integration of the immediate mode GUI crate [`egui`] with the
//! Bevy game engine. Builds off of the [`bevy_egui`] crate.

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
