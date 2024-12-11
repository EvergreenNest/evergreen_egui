//! Containers for grouping widgets together.

use bevy::prelude::World;
use egui::{frame::Prepared, Frame, Layout, Response, Sense, Ui, UiBuilder};

use crate::ui::UiData;

/// Trait for types that can be converted into two halves of a container.
pub trait IntoContainer {
    /// The first half of the container.
    type BeginContainer: BeginContainer;

    /// The second half of the container.
    type EndContainer: EndContainer<Data = <Self::BeginContainer as BeginContainer>::Data>;

    /// Converts this object into two halves of a container.
    fn into_container(self) -> (Self::BeginContainer, Self::EndContainer);
}

impl<C, D> IntoContainer for C
where
    C: BeginContainer<Data = D> + EndContainer<Data = D> + Clone,
    D: UiData,
{
    type BeginContainer = C;
    type EndContainer = C;

    fn into_container(self) -> (Self::BeginContainer, Self::EndContainer) {
        (self.clone(), self)
    }
}

/// The first half of a container.
pub trait BeginContainer: Send + 'static {
    /// The data type returned by [`BeginContainer::begin`].
    type Data: UiData;

    /// Starts a new container and returns the data needed to end it.
    fn begin(self, world: &World, parent: &mut Ui) -> Self::Data;
}

/// The second half of a container.
pub trait EndContainer: Send + 'static {
    /// The data type accepted by [`EndContainer::end`].
    type Data: UiData;

    /// Ends the container and returns the [`egui::Response`] from the container.
    fn end(self, world: &World, parent: &mut Ui, data: Self::Data) -> Response;
}

/// A container that wraps a group of widgets with [`Frame::group`].
#[derive(Clone)]
pub struct Group;

#[doc(hidden)]
pub struct GroupData(Prepared);

impl UiData for GroupData {
    fn ui(&self) -> &egui::Ui {
        &self.0.content_ui
    }

    fn ui_mut(&mut self) -> &mut egui::Ui {
        &mut self.0.content_ui
    }
}

impl BeginContainer for Group {
    type Data = GroupData;

    fn begin(self, _world: &World, parent: &mut Ui) -> Self::Data {
        GroupData(Frame::group(parent.style()).begin(parent))
    }
}

impl EndContainer for Group {
    type Data = GroupData;

    fn end(self, _world: &World, parent: &mut Ui, data: Self::Data) -> Response {
        data.0.end(parent)
    }
}

impl BeginContainer for Layout {
    type Data = Ui;

    fn begin(self, _world: &World, parent: &mut Ui) -> Self::Data {
        parent.new_child(UiBuilder::new().layout(self))
    }
}

impl EndContainer for Layout {
    type Data = Ui;

    fn end(self, _world: &World, parent: &mut Ui, child_ui: Self::Data) -> Response {
        parent.allocate_rect(child_ui.min_rect(), Sense::hover())
    }
}
