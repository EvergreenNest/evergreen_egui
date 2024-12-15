//! Containers for grouping widgets together.

use egui::{
    menu::SubMenu, scroll_area::ScrollAreaOutput, CollapsingHeader, CollapsingResponse, ComboBox,
    Frame, InnerResponse, Layout, Resize, ScrollArea, Ui, UiBuilder,
};

use crate::ui::WorldUi;

/// Trait for types that can be used as containers for grouping widgets together.
pub trait Container {
    /// The type of [`Ui`] that this container provides inside the closure.
    type Ui: ?Sized;

    /// The output type of the container. `R` is the output type of the closure.
    type Out<R>;

    /// Renders this container and calls the given closure with a [`WorldUi`]
    /// that can be used to render UI elements inside the container.
    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R>;
}

impl Container for Layout {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        ui.with_layout(self, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Container for ComboBox {
    type Ui = Ui;
    type Out<R> = InnerResponse<Option<R>>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        self.show_ui(ui, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Container for Resize {
    type Ui = Ui;
    type Out<R> = R;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        self.show(ui, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Container for Frame {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        self.show(ui, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Container for ScrollArea {
    type Ui = Ui;
    type Out<R> = ScrollAreaOutput<R>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        self.show(ui, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Container for CollapsingHeader {
    type Ui = Ui;
    type Out<R> = CollapsingResponse<R>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        self.show(ui, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Container for SubMenu {
    type Ui = Ui;
    type Out<R> = InnerResponse<Option<R>>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        self.show(ui, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

/// [`Container`] that renders `COLS` columns. `COLS` can either be a
/// runtime-specified `usize` or a compile-time-specified [`Const<N>`].
pub struct Columns<COLS>(pub COLS);

impl Container for Columns<usize> {
    type Ui = [Ui];
    type Out<R> = R;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        ui.columns(self.0, move |columns| {
            let ui = WorldUi::new(world, columns);
            f(ui)
        })
    }
}

/// Specifies a constant number of [`Columns`].
pub struct Const<const N: usize>;

impl<const N: usize> Container for Columns<Const<N>> {
    type Ui = [Ui; N];
    type Out<R> = R;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        ui.columns_const::<N, _>(move |columns| {
            let ui = WorldUi::new(world, columns);
            f(ui)
        })
    }
}

/// [`Container`] that renders a maybe-enabled UI.
pub struct Enabled(pub bool);

impl Container for Enabled {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        ui.add_enabled_ui(self.0, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

impl Container for UiBuilder {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        ui.scope_builder(self, |ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}

/// [`Container`] that groups [`Widget`]s together with [`Ui::group`].
///
/// [`Widget`]: crate::widget::Widget
pub struct Group;

impl Container for Group {
    type Ui = Ui;
    type Out<R> = InnerResponse<R>;

    fn show<'world, R>(
        self,
        ui: WorldUi<'world, '_>,
        f: impl FnOnce(WorldUi<'world, '_, Self::Ui>) -> R,
    ) -> Self::Out<R> {
        let (world, ui) = ui.into_parts();
        ui.group(|ui| {
            let ui = WorldUi::new(world, ui);
            f(ui)
        })
    }
}
