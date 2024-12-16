//! Provides types and traits for rendering UI elements inside a [`World`].

use std::{
    hash::Hash,
    ops::{Deref, DerefMut, IndexMut},
};

use bevy_ecs::{
    system::{IntoSystem, RegisteredSystemError, System, SystemInput},
    world::World,
};
use egui::{CollapsingResponse, InnerResponse, Ui, UiBuilder, WidgetText};

use crate::{
    prelude::Container,
    widget::{Draw, IntoWidget, Widget},
};

/// Context for rendering UI elements inside a [`World`].
pub struct WorldUi<'world, 'ui, U: ?Sized = Ui> {
    world: &'world mut World,
    ui: &'ui mut U,
}

impl<'world, 'ui, U: ?Sized> WorldUi<'world, 'ui, U> {
    /// Creates a new instance with the given [`World`] and [`Ui`] instance.
    #[inline]
    pub fn new(world: &'world mut World, ui: &'ui mut U) -> Self {
        WorldUi { world, ui }
    }

    /// Creates a new instance from this with a shorter lifetime.
    #[inline]
    pub fn reborrow(&mut self) -> WorldUi<'_, '_, U> {
        WorldUi {
            world: self.world,
            ui: self.ui,
        }
    }

    /// Returns a reference to the world.
    #[inline]
    pub fn world(&self) -> &World {
        self.world
    }

    /// Returns a mutable reference to the world.
    #[inline]
    pub fn world_mut(&mut self) -> &mut World {
        self.world
    }

    /// Returns references to the world and [`Ui`] instance.
    #[inline]
    pub fn parts(&mut self) -> (&mut World, &mut U) {
        (self.world, self.ui)
    }

    /// Consumes this instance and returns the world and [`Ui`] instance.
    #[inline]
    pub fn into_parts(self) -> (&'world mut World, &'ui mut U) {
        (self.world, self.ui)
    }

    /// Returns a [`WorldUi`] instance for the [`Ui`] instance at the given index.
    #[inline]
    pub fn at<I>(&mut self, index: I) -> WorldUi<'_, '_, U::Output>
    where
        U: IndexMut<I>,
    {
        WorldUi {
            world: self.world,
            ui: &mut self.ui[index],
        }
    }

    /// Calls the given closure with a [`WorldUi`] for each [`Ui`] instance in the
    /// given iterable.
    pub fn for_each<'s, E: 's>(&'s mut self, mut f: impl FnMut(WorldUi<'_, '_, E>))
    where
        &'s mut U: IntoIterator<Item = &'s mut E>,
    {
        for ui in self.ui.into_iter() {
            f(WorldUi {
                world: self.world,
                ui,
            });
        }
    }
}

impl WorldUi<'_, '_, Ui> {
    /// Returns this instance as a single-element slice of [`Ui`] instances,
    /// with a shorter lifetime.
    pub fn as_slice(&mut self) -> WorldUi<'_, '_, [Ui]> {
        WorldUi {
            world: self.world,
            ui: std::slice::from_mut(self.ui),
        }
    }

    /// Returns a reference to the [`Ui`] instance.
    #[inline]
    pub fn ui(&self) -> &Ui {
        self.ui
    }

    /// Returns a mutable reference to the [`Ui`] instance.
    #[inline]
    pub fn ui_mut(&mut self) -> &mut Ui {
        self.ui
    }

    /// Adds a [`Widget`] to this [`Ui`] instance and calls the given
    /// [`Responder`] with the [`egui::Response`] from the widget.
    pub fn add<W: IntoWidget<M>, M>(&mut self, widget: W) -> <W::Widget as Widget>::Out {
        let widget = widget.into_widget();
        widget.draw(self.reborrow())
    }

    /// Runs the given system with this [`Ui`] instance and returns the output.
    pub fn run_cached<I, O, M, S>(
        &mut self,
        system: S,
    ) -> Result<<S::System as System>::Out, RegisteredSystemError<I, O>>
    where
        S: IntoSystem<I, O, M> + 'static,
        I: for<'a> SystemInput<Inner<'a>: From<&'a mut Ui>> + 'static,
        O: 'static,
    {
        self.world
            .run_system_cached_with(system, I::Inner::from(self.ui))
    }

    /// Runs the given system with this [`Ui`] instance and the given extra data,
    /// and returns the output.
    pub fn run_cached_with<'s: 'e, 'e, S, E, O, M>(
        &'s mut self,
        system: S,
        extra: E::Inner<'e>,
    ) -> Result<<S::System as System>::Out, RegisteredSystemError<Draw<'static, E>, O>>
    where
        S: IntoSystem<Draw<'static, E>, O, M> + 'static,
        E: SystemInput + 'static,
        O: 'static,
    {
        self.world
            .run_system_cached_with(system, Draw::new(self.ui, extra))
    }

    /// Shows a [`Container`] and calls the given closure with a [`WorldUi`] that
    /// can be used to render UI elements inside the container.
    pub fn show<C: Container, R>(
        &mut self,
        container: C,
        add_contents: impl FnOnce(WorldUi<'_, '_, C::Ui>) -> R,
    ) -> C::Out<R> {
        container.show(self.reborrow(), add_contents)
    }

    /// [`Ui::group`] with [`World`] access.
    pub fn group<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.group(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::scope`] with [`World`] access.
    pub fn scope<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.scope(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::scope_builder`] with [`World`] access.
    pub fn scope_builder<R>(
        &mut self,
        ui_builder: UiBuilder,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.scope_builder(ui_builder, |ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::collapsing`] with [`World`] access.
    pub fn collapsing<R>(
        &mut self,
        heading: impl Into<WidgetText>,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> CollapsingResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.collapsing(heading, |ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::indent`] with [`World`] access.
    pub fn indent<R>(
        &mut self,
        id_salt: impl Hash,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.indent(id_salt, |ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::horizontal`] with [`World`] access.
    pub fn horizontal<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.horizontal(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::horizontal_centered`] with [`World`] access.
    pub fn horizontal_centered<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.horizontal_centered(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::horizontal_top`] with [`World`] access.
    pub fn horizontal_top<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.horizontal_top(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::horizontal_wrapped`] with [`World`] access.
    pub fn horizontal_wrapped<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.horizontal_wrapped(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::vertical`] with [`World`] access.
    pub fn vertical<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.vertical(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::vertical_centered`] with [`World`] access.
    pub fn vertical_centered<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.vertical_centered(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::vertical_top`] with [`World`] access.
    pub fn vertical_centered_justified<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.vertical_centered_justified(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::centered_and_justified`] with [`World`] access.
    pub fn centered_and_justified<R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<R> {
        let (world, ui) = self.reborrow().into_parts();
        ui.centered_and_justified(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::group`] with [`World`] access.
    pub fn columns<R>(
        &mut self,
        columns: usize,
        add_contents: impl FnOnce(WorldUi<'_, '_, [Ui]>) -> R,
    ) -> R {
        let (world, ui) = self.reborrow().into_parts();
        ui.columns(columns, |ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::columns`] with a constant number of columns.
    pub fn columns_const<const NUM_COL: usize, R>(
        &mut self,
        add_contents: impl FnOnce(WorldUi<'_, '_, [Ui; NUM_COL]>) -> R,
    ) -> R {
        let (world, ui) = self.reborrow().into_parts();
        ui.columns_const(|ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }

    /// [`Ui::menu_button`] with [`World`] access.
    pub fn menu_button<R>(
        &mut self,
        title: impl Into<WidgetText>,
        add_contents: impl FnOnce(WorldUi<'_, '_, Ui>) -> R,
    ) -> InnerResponse<Option<R>> {
        let (world, ui) = self.reborrow().into_parts();
        ui.menu_button(title, |ui| {
            let ui = WorldUi::new(world, ui);
            add_contents(ui)
        })
    }
}

impl WorldUi<'_, '_, [Ui]> {
    /// Returns a reference to the [`Ui`] instances.
    #[inline]
    pub fn uis(&self) -> &[Ui] {
        self.ui
    }

    /// Returns a mutable reference to the [`Ui`] instances.
    #[inline]
    pub fn uis_mut(&mut self) -> &mut [Ui] {
        self.ui
    }

    /// Returns a reference to the [`Ui`] instance at the given index.
    #[inline]
    pub fn ui(&self, index: usize) -> Option<&Ui> {
        self.ui.get(index)
    }

    /// Returns a mutable reference to the [`Ui`] instance at the given index.
    #[inline]
    pub fn ui_mut(&mut self, index: usize) -> Option<&mut Ui> {
        self.ui.get_mut(index)
    }
}

impl<U: ?Sized> Deref for WorldUi<'_, '_, U> {
    type Target = World;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.world
    }
}

impl<U: ?Sized> DerefMut for WorldUi<'_, '_, U> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.world
    }
}
