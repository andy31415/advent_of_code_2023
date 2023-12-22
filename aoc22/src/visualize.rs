use aoc22::Brick;
use bevy::{app::AppExit, audio::Decodable, prelude::*};

#[derive(Component, Debug)]
struct BrickDisplay {
    brick: Brick,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_floor, load_input, load_camera, load_light))
        .add_systems(Update, handle_exit)
        .run();
}

fn load_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
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

const SCALE: f32 = 0.2;

fn load_input(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (idx, brick) in aoc22::parse_input(include_str!("../example.txt")).into_iter().enumerate() {
        // figure out ranges for the brick
        let x = (
            brick.start.x.min(brick.end.x) as f32,
            brick.start.x.max(brick.end.x) as f32,
        );
        let y = (
            brick.start.y.min(brick.end.y) as f32,
            brick.start.y.max(brick.end.y) as f32,
        );
        let z = (
            brick.start.z.min(brick.end.z) as f32,
            brick.start.z.max(brick.end.z) as f32,
        );

        // Bevy has Y up and xz the plane, so flip
        let (x, y, z) = (x, z, y);

        // everything goes -1 to top
        let lower = Vec3::new(x.0 - 1.0, y.0 - 1.0, z.0 - 1.0) * SCALE;
        let upper = Vec3::new(x.1, y.1, z.1) * SCALE;
        let h = (((idx % 20)) * (360/20)) as f32;
        
        dbg!(h);
        
        let item = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(lower, upper))),
            material: materials.add(
                Color::hsl(h, 1.0, 0.5).into()
            ),
            ..default()
        };
        commands.spawn((BrickDisplay { brick }, item));
    }
}

fn handle_exit(input: Res<Input<KeyCode>>, mut quit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        quit.send(AppExit);
    }
}
