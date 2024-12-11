//! Types and traits for responding to [`Response`]s from [`Widget`]s.
//!
//! [`Widget`]: crate::widget::Widget

use std::marker::PhantomData;

use bevy::prelude::{In, IntoSystem, World};
use egui::Response;

/// Trait for types that can respond to a [`Response`] from a [`Widget`].
///
/// [`Widget`]: crate::widget::Widget
pub trait Responder: Send + 'static {
    /// Responds to a [`Response`] from a [`Widget`].
    ///
    /// [`Widget`]: crate::widget::Widget
    fn respond(self, world: &mut World, response: Response);
}

impl Responder for () {
    fn respond(self, _world: &mut World, _response: Response) {}
}

/// Conversion trait to turn something into a [`Responder`].
pub trait IntoResponder<M> {
    /// The type of [`Responder`] that this conversion produces.
    type Responder: Responder;

    /// Converts this value into a [`Responder`].
    fn into_responder(self) -> Self::Responder;
}

/// All [`Responder`]s implicitly implement [`IntoResponder`].
impl<R: Responder> IntoResponder<()> for R {
    type Responder = R;

    fn into_responder(self) -> Self::Responder {
        self
    }
}

#[doc(hidden)]
pub struct SystemResponder<S, Marker>(S, PhantomData<fn() -> Marker>)
where
    S: IntoSystem<In<Response>, (), Marker> + Send + 'static,
    Marker: 'static;

impl<S, Marker> Responder for SystemResponder<S, Marker>
where
    S: IntoSystem<In<Response>, (), Marker> + Send + 'static,
    Marker: 'static,
{
    fn respond(self, world: &mut World, response: Response) {
        let _ = world.run_system_cached_with(self.0, response);
    }
}

#[doc(hidden)]
pub struct SystemResponderMarker;

impl<S, Marker> IntoResponder<(SystemResponderMarker, Marker)> for S
where
    S: IntoSystem<In<Response>, (), Marker> + Send + 'static,
    Marker: 'static,
{
    type Responder = SystemResponder<S, Marker>;

    fn into_responder(self) -> Self::Responder {
        const {
            assert!(
                size_of::<S>() == 0,
                "Non-ZST systems (e.g. capturing closures, function pointers) cannot be used directly.",
            );
        }
        SystemResponder(self, PhantomData)
    }
}
