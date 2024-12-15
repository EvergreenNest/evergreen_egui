use bevy::{
    app::{App, Update},
    prelude::{In, Res, Resource, World},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_egui_commands::{ctx::WorldCtxExt, prelude::*};
use egui::{Button, CentralPanel, Label, Layout, Response, SidePanel, TopBottomPanel};

#[derive(Resource, Default)]
pub struct Counter(i32);

pub fn render(world: &mut World) {
    let Some(mut ctx) = world.try_ctx_mut() else {
        return;
    };

    ctx.show(TopBottomPanel::top("top"), |mut ui| {
        ui.add(Label::new("Top panel"));
    });
    ctx.show(SidePanel::left("left"), |mut ui| {
        ui.show(Group, |mut ui| {
            ui.add(Label::new("Left panel"));
        });
    });
    ctx.show(SidePanel::right("right"), |mut ui| {
        ui.show(Layout::bottom_up(egui::Align::Min), |mut ui| {
            ui.add(Label::new("Right panel"));
        });
    });
    ctx.show(CentralPanel::default(), |mut ui| {
        if ui.add(Button::new("Hello world!")).clicked() {
            ui.resource_mut::<Counter>().0 += 1;
        }

        fn my_widget(mut draw: Draw, counter: Res<Counter>) -> Response {
            draw.label(format!("Clicked: {}", counter.0))
        }
        ui.run_cached(my_widget).unwrap();
        fn my_widget_with_data(mut draw: Draw<In<i32>>, counter: Res<Counter>) -> Response {
            let In(data) = *draw.extra();
            draw.label(format!("Total: {}", counter.0 + data))
        }
        ui.run_cached_with(my_widget_with_data, 5).unwrap();
        ui.show(Columns(Const::<5>), |mut ui| {
            for i in 0..5 {
                let mut ui = ui.at(i);
                ui.add(Label::new(i.to_string()));
            }
        })
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<Counter>()
        .add_systems(Update, render)
        .run();
}
