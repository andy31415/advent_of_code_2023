use bevy::{app::AppExit, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, handle_exit)
        .run();
}

fn handle_exit(input: Res<Input<KeyCode>>, mut quit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        quit.send(AppExit);
    }
}
