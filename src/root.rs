//! Types and traits for creating root containers with which to build UIs.

use bevy_ecs::world::World;
use egui::{Area, CentralPanel, Context, InnerResponse, SidePanel, TopBottomPanel, Ui, Window};

use crate::ui::WorldUi;

/// Trait for types that can be used as root containers (e.g. windows, panels).
pub trait Root {
    /// The type of [`Ui`] that this root container provides.
    type Ui: ?Sized;

    /// The output type of the root container. `R` is the output type of the
    /// closure.
    type Out<R>;

    /// Shows this root container and calls the given closure with a [`WorldUi`]
    /// that can be used to render UI elements inside the root.
    fn show<'world, R>(
        self,
        world: &'world mut World,
        ctx: &Context,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R>;
}

impl Root for CentralPanel {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        world: &'world mut World,
        ctx: &Context,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        self.show(ctx, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Root for SidePanel {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        world: &'world mut World,
        ctx: &Context,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        self.show(ctx, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Root for TopBottomPanel {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        world: &'world mut World,
        ctx: &Context,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        self.show(ctx, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Root for Window<'_> {
    type Ui = Ui;
    type Out<R> = Option<InnerResponse<Option<R>>>;

    fn show<'world, R>(
        self,
        world: &'world mut World,
        ctx: &Context,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        self.show(ctx, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Root for Area {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        world: &'world mut World,
        ctx: &Context,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        self.show(ctx, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}
