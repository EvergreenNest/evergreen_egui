//! Provides types and traits for rendering root containers in a given [`World`].

use bevy_ecs::world::World;
use bevy_egui::EguiContexts;
use bevy_log::warn_once;
use egui::Context;

use crate::{root::Root, ui::WorldUi};

/// Context for rendering root containers in a given [`World`].
pub struct WorldCtx<'world> {
    world: &'world mut World,
    ctx: Context,
}

impl<'world> WorldCtx<'world> {
    /// Creates a new instance with the given [`World`] using the [`Context`]
    /// that corresponds to the primary window.
    pub fn new(world: &'world mut World) -> Option<Self> {
        fn get_ctx(mut ctxs: EguiContexts) -> Option<Context> {
            ctxs.try_ctx_mut().cloned()
        }
        let Ok(Some(ctx)) = world.run_system_cached(get_ctx) else {
            warn_once!("No egui context found");
            return None;
        };
        Some(Self { world, ctx })
    }

    /// Shows a root container and calls the given closure with a [`WorldUi`]
    /// that can be used to render UI elements inside the root.
    pub fn show<Ro: Root, R>(
        &mut self,
        root: Ro,
        f: impl FnOnce(WorldUi<'_, '_, Ro::Ui>) -> R,
    ) -> Ro::Out<R> {
        root.show(self.world, &self.ctx, f)
    }
}

/// [`World`] extension trait for fetching [`WorldCtx`] instances used to render
/// root containers.
pub trait WorldCtxExt {
    /// Tries to create a [`WorldCtx`] instance for the given [`World`]
    /// targeting the primary window.
    fn try_ctx_mut(&mut self) -> Option<WorldCtx<'_>>;
}

impl WorldCtxExt for World {
    fn try_ctx_mut(&mut self) -> Option<WorldCtx<'_>> {
        fn get_ctx(mut ctxs: EguiContexts) -> Option<Context> {
            ctxs.try_ctx_mut().cloned()
        }

        self.run_system_cached(get_ctx)
            .ok()
            .flatten()
            .map(|ctx| WorldCtx { world: self, ctx })
    }
}
