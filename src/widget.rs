//! Widgets for use in egui UIs.

use std::{marker::PhantomData, sync::Arc};

use bevy::prelude::{InMut, IntoSystem, ReadOnlySystem, Resource, System, SystemInput, World};
use egui::{Response, Ui};
use parking_lot::Mutex;

/// Trait for types that can be rendered as a UI widget.
pub trait Widget: Send + 'static {
    /// Draws this widget to the given [`Ui`].
    fn draw(self, world: &World, ui: &mut Ui) -> Response;
}

#[doc(hidden)]
pub struct EguiWidget<W>(W);

impl<W> Widget for EguiWidget<W>
where
    W: egui::Widget + Send + 'static,
{
    fn draw(self, _world: &World, ui: &mut Ui) -> Response {
        ui.add(self.0)
    }
}

#[derive(Resource)]
struct CachedSystemArc<In: SystemInput, Out, Marker, S: IntoSystem<In, Out, Marker>>(
    Arc<Mutex<S::System>>,
    PhantomData<fn(In) -> (Out, Marker)>,
);

#[doc(hidden)]
pub struct SystemWidget<S, Marker>(Arc<Mutex<S::System>>, PhantomData<fn() -> Marker>)
where
    S: IntoSystem<InMut<'static, Ui>, Response, Marker, System: ReadOnlySystem> + Send + 'static,
    Marker: 'static;

impl<S, Marker> Widget for SystemWidget<S, Marker>
where
    S: IntoSystem<InMut<'static, Ui>, Response, Marker, System: ReadOnlySystem> + Send + 'static,
    Marker: 'static,
{
    fn draw(self, world: &World, ui: &mut Ui) -> Response {
        self.0.lock().run_readonly(ui, world)
    }
}

/// Conversion trait to turn something into a [`Widget`].
pub trait IntoWidget<M>: Send + 'static {
    /// The type of [`Widget`] that this conversion produces.
    type Widget: Widget;

    /// Converts this value into a [`Widget`].
    fn into_widget(self, world: &mut World) -> Self::Widget;
}

impl<W: Widget> IntoWidget<()> for W {
    type Widget = W;

    fn into_widget(self, _world: &mut World) -> Self::Widget {
        self
    }
}

#[doc(hidden)]
pub struct EguiWidgetMarker;

impl<W> IntoWidget<EguiWidgetMarker> for W
where
    W: egui::Widget + Send + 'static,
{
    type Widget = EguiWidget<W>;

    fn into_widget(self, _world: &mut World) -> Self::Widget {
        EguiWidget(self)
    }
}

#[doc(hidden)]
pub struct SystemWidgetMarker;

impl<S, Marker> IntoWidget<(SystemWidgetMarker, Marker)> for S
where
    S: IntoSystem<InMut<'static, Ui>, Response, Marker, System: ReadOnlySystem> + Send + 'static,
    Marker: 'static,
{
    type Widget = SystemWidget<S, Marker>;

    fn into_widget(self, world: &mut World) -> Self::Widget {
        const {
            assert!(
                size_of::<S>() == 0,
                "Non-ZST systems (e.g. capturing closures, function pointers) cannot be used directly.",
            );
        }
        if let Some(system) =
            world.get_resource::<CachedSystemArc<InMut<'static, Ui>, Response, Marker, S>>()
        {
            SystemWidget(Arc::clone(&system.0), PhantomData)
        } else {
            let mut system = IntoSystem::into_system(self);
            system.initialize(world);
            let system = Arc::new(Mutex::new(system));
            world.insert_resource::<CachedSystemArc<InMut<'static, Ui>, Response, Marker, S>>(
                CachedSystemArc(Arc::clone(&system), PhantomData),
            );
            SystemWidget(system, PhantomData)
        }
    }
}
