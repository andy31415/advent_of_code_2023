use aoc22::Brick;
use bevy::{app::AppExit, prelude::*};

#[derive(Component, Debug)]
struct BrickDisplay {
    brick: Brick,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_floor, load_input, load_camera))
        .add_systems(Update, handle_exit)
        .run();
}

fn load_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn load_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::GRAY.into()),
        ..default()
    };

    commands.spawn(floor);
}

fn load_input(mut commands: Commands) {
    for brick in aoc22::parse_input(include_str!("../example.txt")) {
        // TODO: how do I place some form of brick here in space?
        // I have the coordinates...
        commands.spawn(BrickDisplay { brick });
    }
}

fn handle_exit(input: Res<Input<KeyCode>>, mut quit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        quit.send(AppExit);
    }
}
