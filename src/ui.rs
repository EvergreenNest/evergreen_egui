//! Provides types and traits for rendering UI elements inside a [`World`].

use std::ops::{Deref, DerefMut, IndexMut};

use bevy_ecs::{
    system::{IntoSystem, RegisteredSystemError, System, SystemInput},
    world::World,
};
use egui::Ui;

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
    pub fn run_cached<S, O, M>(
        &mut self,
        system: S,
    ) -> Result<<S::System as System>::Out, RegisteredSystemError<Draw<'static>, O>>
    where
        S: IntoSystem<Draw<'static>, O, M> + 'static,
        O: 'static,
    {
        self.run_cached_with(system, ())
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
        f: impl FnOnce(WorldUi<'_, '_, C::Ui>) -> R,
    ) -> C::Out<R> {
        container.show(self.reborrow(), f)
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
