use bevy::prelude::Resource;
use downcast_rs::{impl_downcast, Downcast};
use egui::Ui;

/// A stack of [`Ui`]s plus associated data for cleanup post removal.
#[derive(Resource, Default)]
pub struct UiStack {
    stack: Vec<Box<dyn UiData>>,
}

impl UiStack {
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, ui: impl UiData) {
        self.stack.push(Box::new(ui));
    }

    pub fn pop<T: UiData>(&mut self) -> Option<T> {
        self.stack
            .pop()
            .map(|ui| *ui.downcast().unwrap_or_else(|_| panic!("downcast failed")))
    }

    pub fn top_mut(&mut self) -> Option<&mut Ui> {
        self.stack.last_mut().map(|ui| ui.ui_mut())
    }
}

/// Trait for types that contain a [`Ui`] and additional data needed for cleanup.
pub trait UiData: Downcast + Send + Sync + 'static {
    /// Returns a reference to the "innermost" contained [`Ui`].
    ///
    /// "Innermost" refers to where new widgets are added.
    fn ui(&self) -> &egui::Ui;

    /// Returns a mutable reference to the "innermost" contained [`Ui`].
    ///
    /// "Innermost" refers to where new widgets are added.
    fn ui_mut(&mut self) -> &mut egui::Ui;
}

impl_downcast!(UiData);

/// A plain [`Ui`] without any additional data needed for cleanup.
impl UiData for Ui {
    fn ui(&self) -> &egui::Ui {
        self
    }

    fn ui_mut(&mut self) -> &mut egui::Ui {
        self
    }
}
