//! [`Command`]s for rendering [`Widget`]s and [`IntoContainer`]s inside
//! [`IntoRoot`]s.

use bevy::{
    ecs::system::SystemState,
    log::warn,
    prelude::{Command, Commands, Mut, World},
};
use bevy_egui::EguiContexts;

use crate::{
    prelude::{BeginContainer, EndContainer, IntoContainer, IntoResponder},
    responder::Responder,
    root::{BeginRoot, EndRoot, IntoRoot},
    ui::UiStack,
    widget::{IntoWidget, Widget},
};

/// [`Commands`] extension for queuing commands that render [`IntoRoot`]s.
pub trait RootCommands {
    /// Queues a [`IntoRoot`] to be rendered. The given closure will be called
    /// with a [`UiCommands`] that can be used to queue commands that render
    /// [`Widget`]s and [`IntoContainer`]s inside the root.
    fn show(&mut self, root: impl IntoRoot, f: impl FnOnce(UiCommands)) -> &mut Self;
}

impl RootCommands for Commands<'_, '_> {
    fn show(&mut self, container: impl IntoRoot, f: impl FnOnce(UiCommands)) -> &mut Self {
        let (begin_root, end_root) = container.into_root();
        self.queue(StartRootCommand(begin_root));
        f(UiCommands {
            commands: self.reborrow(),
        });
        self.queue(EndRootCommand(end_root));
        self
    }
}

/// [`Commands`] wrapper for queuing commands that render [`Widget`]s and
/// [`IntoContainer`]s.
pub struct UiCommands<'w, 's> {
    commands: Commands<'w, 's>,
}

impl UiCommands<'_, '_> {
    /// Returns a [`UiCommands`] with a smaller lifetime.
    pub fn reborrow(&mut self) -> UiCommands<'_, '_> {
        UiCommands {
            commands: self.commands.reborrow(),
        }
    }

    /// Queues a [`Widget`] to be rendered. If a [`Responder`] is provided, it
    /// will be called with the [`egui::Response`] from the widget.
    pub fn add<WM: 'static, RM>(
        &mut self,
        widget: impl IntoWidget<WM>,
        respond: impl IntoResponder<RM>,
    ) -> &mut Self {
        self.commands.queue(WidgetCommand {
            widget,
            respond: respond.into_responder(),
            _marker: std::marker::PhantomData,
        });
        self
    }

    /// Queues an [`IntoContainer`] to be rendered. If a [`Responder`] is
    /// provided, it will be called with the [`egui::Response`] from the
    /// container.
    pub fn show<RM>(
        &mut self,
        container: impl IntoContainer,
        respond: impl IntoResponder<RM>,
        f: impl FnOnce(UiCommands),
    ) -> &mut Self {
        let (start, end) = container.into_container();
        self.commands.queue(StartContainerCommand(start));
        f(self.reborrow());
        self.commands.queue(EndContainerCommand {
            end,
            respond: respond.into_responder(),
        });
        self
    }
}

/// [`Command`] that runs the [`StartRoot`] half of a root.
struct StartRootCommand<R: BeginRoot>(R);

impl<R: BeginRoot> Command for StartRootCommand<R> {
    fn apply(self, world: &mut World) {
        let mut state = SystemState::<EguiContexts>::new(world);
        let mut ctxs = state.get_mut(world);
        let Some(ctx) = ctxs.try_ctx_mut() else {
            warn!("No egui context found");
            return;
        };
        let ctx = ctx.clone();
        let data = self.0.begin(world, &ctx);
        let mut stack = UiStack::default();
        stack.push(data);
        world.insert_resource(stack);
    }
}

/// [`Command`] that runs the [`EndRoot`] half of a root.
struct EndRootCommand<R: EndRoot>(R);

impl<R: EndRoot> Command for EndRootCommand<R> {
    fn apply(self, world: &mut World) {
        let Some(mut stack) = world.remove_resource::<UiStack>() else {
            warn!("No UiStack found");
            return;
        };
        if stack.len() != 1 {
            warn!("Container was not ended");
        }
        let Some(ui) = stack.pop() else {
            warn!("No Root was started");
            return;
        };
        self.0.end(world, ui);
    }
}

struct StartContainerCommand<C: BeginContainer>(C);

impl<C: BeginContainer> Command for StartContainerCommand<C> {
    fn apply(self, world: &mut World) {
        if !world.contains_resource::<UiStack>() {
            warn!("No UiStack found");
            return;
        }

        world.resource_scope(|world, mut stack: Mut<UiStack>| {
            let Some(parent) = stack.top_mut() else {
                warn!("No parent Ui found");
                return;
            };
            let this = self.0.begin(world, parent);
            stack.push(this);
        });
    }
}

struct EndContainerCommand<C: EndContainer, R: Responder> {
    end: C,
    respond: R,
}

impl<C: EndContainer, R: Responder> Command for EndContainerCommand<C, R> {
    fn apply(self, world: &mut World) {
        if !world.contains_resource::<UiStack>() {
            warn!("No UiStack found");
            return;
        }

        world.resource_scope(|world, mut stack: Mut<UiStack>| {
            let Some(this) = stack.pop() else {
                warn!("No Container was started");
                return;
            };
            let Some(parent) = stack.top_mut() else {
                warn!("No parent Ui found");
                return;
            };
            let response = self.end.end(world, parent, this);
            self.respond.respond(world, response);
        });
    }
}

struct WidgetCommand<W: IntoWidget<M>, R: Responder, M: 'static> {
    widget: W,
    respond: R,
    _marker: std::marker::PhantomData<fn() -> M>,
}

impl<W: IntoWidget<M>, R: Responder, M: 'static> Command for WidgetCommand<W, R, M> {
    fn apply(self, world: &mut World) {
        if !world.contains_resource::<UiStack>() {
            warn!("No UiStack found");
            return;
        }

        world.resource_scope(|world, mut stack: Mut<UiStack>| {
            let widget = self.widget.into_widget(world);

            let Some(top) = stack.top_mut() else {
                warn!("No Ui found");
                return;
            };
            let resp = widget.draw(world, top);
            self.respond.respond(world, resp);
        });
    }
}
