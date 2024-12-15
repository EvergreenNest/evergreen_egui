//! Widgets for use in egui UIs.

use std::ops::{Deref, DerefMut};

use bevy_ecs::system::SystemInput;
use egui::{Response, Ui};
use variadics_please::all_tuples;

use crate::ui::WorldUi;

/// Trait for types that can be rendered as a UI widget.
pub trait Widget {
    /// The type of value that this widget returns when drawn.
    type Out;

    /// Draws this widget to the given [`Ui`].
    fn draw(self, ui: WorldUi) -> Self::Out;
}

macro_rules! impl_widget_tuple {
    ($(#[$meta:meta])* $($name:ident),*) => {
        $(#[$meta])*
        impl<$($name: Widget),*> Widget for ($($name,)*) {
            type Out = ($($name::Out,)*);

            #[allow(clippy::unused_unit)]
            #[allow(non_snake_case, unused_variables, unused_mut)]
            fn draw(self, mut ui: WorldUi) -> Self::Out {
                let ($($name,)*) = self;
                ($($name.draw(ui.reborrow()),)*)
            }
        }
    };
}

all_tuples!(
    #[doc(fake_variadic)]
    impl_widget_tuple,
    0,
    16,
    W
);

#[doc(hidden)]
pub struct EguiWidget<W>(W);

impl<W> Widget for EguiWidget<W>
where
    W: egui::Widget + Send + 'static,
{
    type Out = Response;

    fn draw(self, mut ui: WorldUi) -> Self::Out {
        ui.ui_mut().add(self.0)
    }
}

/// [`SystemInput`] for drawing UI elements. Extra data can be passed as well.
pub struct Draw<'ui, E = ()> {
    /// The [`Ui`] instance to draw to.
    pub ui: &'ui mut Ui,
    /// Extra data to pass to the widget system.
    pub extra: E,
}

impl<E: SystemInput> SystemInput for Draw<'_, E> {
    type Param<'i> = Draw<'i, E::Param<'i>>;
    type Inner<'i> = Draw<'i, E::Inner<'i>>;

    fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
        Draw {
            ui: this.ui,
            extra: E::wrap(this.extra),
        }
    }
}

impl<'ui, E> Draw<'ui, E> {
    /// Creates a new [`Draw`] instance with the given [`Ui`] and extra data.
    pub fn new(ui: &'ui mut Ui, extra: E) -> Self {
        Draw { ui, extra }
    }

    /// Returns a reference to the [`Ui`] instance.
    pub fn ui(&mut self) -> &mut Ui {
        self.ui
    }

    /// Returns a mutable reference to the extra data.
    pub fn extra(&mut self) -> &mut E {
        &mut self.extra
    }

    /// Returns mutable references to the [`Ui`] instance and the extra data.
    pub fn as_parts(&mut self) -> (&mut Ui, &mut E) {
        (self.ui, &mut self.extra)
    }

    /// Consumes this [`Draw`] instance and returns the [`Ui`] instance and the extra data.
    pub fn into_parts(self) -> (&'ui mut Ui, E) {
        (self.ui, self.extra)
    }
}

impl<T> Deref for Draw<'_, T> {
    type Target = Ui;

    fn deref(&self) -> &Self::Target {
        self.ui
    }
}

impl<T> DerefMut for Draw<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ui
    }
}

/// Conversion trait to turn something into a [`Widget`].
pub trait IntoWidget<M> {
    /// The type of [`Widget`] that this conversion produces.
    type Widget: Widget;

    /// Converts this value into a [`Widget`].
    fn into_widget(self) -> Self::Widget;
}

impl<W: Widget> IntoWidget<()> for W {
    type Widget = W;

    fn into_widget(self) -> Self::Widget {
        self
    }
}

macro_rules! impl_into_widget_tuple {
    ($(#[$meta:meta])* $(($W:ident, $w:ident, $M:ident)),*) => {
        $(#[$meta])*
        impl<$($W: IntoWidget<$M>, $M),*> IntoWidget<($($M,)*)> for ($($W,)*) {
            type Widget = ($($W::Widget,)*);

            fn into_widget(self) -> Self::Widget {
                let ($($w,)*) = self;
                ($($w.into_widget(),)*)
            }
        }
    };
}

all_tuples!(
    #[doc(fake_variadic)]
    impl_into_widget_tuple,
    1,
    16,
    W,
    w,
    M
);

#[doc(hidden)]
pub struct EguiWidgetMarker;

impl<W> IntoWidget<EguiWidgetMarker> for W
where
    W: egui::Widget + Send + 'static,
{
    type Widget = EguiWidget<W>;

    fn into_widget(self) -> Self::Widget {
        EguiWidget(self)
    }
}
