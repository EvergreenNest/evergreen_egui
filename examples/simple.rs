use bevy::{
    app::{App, Update},
    prelude::{Commands, In, InMut, Res, ResMut, Resource},
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_egui_commands::prelude::*;
use egui::{Button, CentralPanel, Label, Layout, Response, SidePanel, TopBottomPanel, Ui};

#[derive(Resource, Default)]
pub struct Counter(i32);

pub fn render(mut commands: Commands) {
    commands.show(TopBottomPanel::top("top"), |mut ui| {
        ui.add(Label::new("Top panel"), ());
    });
    commands.show(SidePanel::left("left"), |mut ui| {
        ui.show(Group, (), |mut ui| {
            ui.add(Label::new("Left panel"), ());
        });
    });
    commands.show(SidePanel::right("right"), |mut ui| {
        ui.show(Layout::bottom_up(egui::Align::Min), (), |mut ui| {
            ui.add(Label::new("Right panel"), ());
        });
    });
    commands.show(CentralPanel::default(), |mut ui| {
        ui.add(
            Button::new("Hello world!"),
            |In(response): In<Response>, mut counter: ResMut<Counter>| {
                if response.clicked() {
                    counter.0 += 1;
                }
            },
        );
        fn my_widget(InMut(ui): InMut<Ui>, counter: Res<Counter>) -> Response {
            ui.label(format!("Clicked: {}", counter.0))
        }
        ui.add(my_widget, ());
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
