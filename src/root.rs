//! Types and traits for creating root containers with which to build UIs.

use bevy::prelude::World;
use egui::{
    panel::{PreparedCentralPanel, PreparedSidePanel, PreparedTopBottomPanel},
    CentralPanel, Context, SidePanel, TopBottomPanel,
};

use crate::ui::UiData;

/// Conversion trait to turn something into a root container.
pub trait IntoRoot {
    /// The first half of the root container.
    type Begin: BeginRoot;

    /// The second half of the root container.
    type End: EndRoot<Data = <Self::Begin as BeginRoot>::Data>;

    /// Converts this value into two halves of a root container.
    fn into_root(self) -> (Self::Begin, Self::End);
}

impl<R, RD> IntoRoot for R
where
    R: BeginRoot<Data = RD> + EndRoot<Data = RD> + Clone,
    RD: UiData,
{
    type Begin = R;
    type End = R;

    fn into_root(self) -> (Self::Begin, Self::End) {
        (self.clone(), self)
    }
}

/// The first half of a [`IntoRoot`] chain.
pub trait BeginRoot: Send + 'static {
    /// The type of data that will be stored while rendering the root.
    type Data: UiData;

    /// Begins rendering this root container.
    fn begin(self, world: &World, ctx: &Context) -> Self::Data;
}

/// The second half of a [`IntoRoot`] chain.
pub trait EndRoot: Send + 'static {
    /// The type of data that was stored while rendering the root.
    type Data: UiData;

    /// Finishes rendering this root container.
    fn end(self, world: &World, data: Self::Data);
}

impl UiData for PreparedCentralPanel {
    fn ui(&self) -> &egui::Ui {
        self.content_ui()
    }

    fn ui_mut(&mut self) -> &mut egui::Ui {
        self.content_ui_mut()
    }
}

impl IntoRoot for CentralPanel {
    type Begin = CentralPanel;
    type End = EndCentralPanel;

    fn into_root(self) -> (Self::Begin, Self::End) {
        (self, EndCentralPanel)
    }
}

/// [`EndRoot`] for [`CentralPanel`].
pub struct EndCentralPanel;

impl BeginRoot for CentralPanel {
    type Data = PreparedCentralPanel;

    fn begin(self, _world: &World, ctx: &Context) -> Self::Data {
        self.begin(ctx)
    }
}

impl EndRoot for EndCentralPanel {
    type Data = PreparedCentralPanel;

    fn end(self, _world: &World, data: Self::Data) {
        data.end();
    }
}

impl UiData for PreparedSidePanel {
    fn ui(&self) -> &egui::Ui {
        self.content_ui()
    }

    fn ui_mut(&mut self) -> &mut egui::Ui {
        self.content_ui_mut()
    }
}

impl IntoRoot for SidePanel {
    type Begin = Self;
    type End = EndSidePanel;

    fn into_root(self) -> (Self::Begin, Self::End) {
        (self, EndSidePanel)
    }
}

/// [`EndRoot`] for [`SidePanel`].
pub struct EndSidePanel;

impl BeginRoot for SidePanel {
    type Data = PreparedSidePanel;

    fn begin(self, _world: &World, ctx: &Context) -> Self::Data {
        self.begin(ctx)
    }
}

impl EndRoot for EndSidePanel {
    type Data = PreparedSidePanel;

    fn end(self, _world: &World, data: Self::Data) {
        data.end();
    }
}

impl UiData for PreparedTopBottomPanel {
    fn ui(&self) -> &egui::Ui {
        self.content_ui()
    }

    fn ui_mut(&mut self) -> &mut egui::Ui {
        self.content_ui_mut()
    }
}

impl IntoRoot for TopBottomPanel {
    type Begin = TopBottomPanel;
    type End = EndTopBottomPanel;

    fn into_root(self) -> (Self::Begin, Self::End) {
        (self, EndTopBottomPanel)
    }
}

/// [`EndRoot`] for [`TopBottomPanel`].
pub struct EndTopBottomPanel;

impl BeginRoot for TopBottomPanel {
    type Data = PreparedTopBottomPanel;

    fn begin(self, _world: &World, ctx: &Context) -> Self::Data {
        self.begin(ctx)
    }
}

impl EndRoot for EndTopBottomPanel {
    type Data = PreparedTopBottomPanel;

    fn end(self, _world: &World, data: Self::Data) {
        data.end();
    }
}
