use aoc22::Building;
use bevy::{
    app::AppExit,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    utils::HashMap,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component, Debug)]
struct BrickDisplay {}

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .init_resource::<BrickColors>()
        .add_systems(Startup, (load_floor, load_input, load_camera, load_light))
        .add_systems(Startup, faster_present)
        .add_systems(Update, (handle_exit, pan_orbit_camera))
        .add_systems(Update, reload_data);

    #[cfg(feature = "fps")] // debug/dev builds only
    {
        app.add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
        ));
    }

    app.run();
}

#[derive(Resource, Default, Debug)]
struct BrickColors {
    colors: HashMap<usize, Handle<StandardMaterial>>,
}

impl BrickColors {
    fn get(
        &mut self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        idx: usize,
    ) -> Handle<StandardMaterial> {
        if let Some(v) = self.colors.get(&idx) {
            return v.clone();
        }

        let h = ((idx * 53) % 360) as f32;

        let material = materials.add(StandardMaterial {
            base_color: Color::hsl(h, 1.0, 0.5),
            double_sided: false,
            ..default()
        });
        self.colors.insert(idx, material.clone());
        material
    }
}

#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

fn faster_present(mut windows: Query<&mut Window>) {
    windows.get_single_mut().expect("have window").present_mode = PresentMode::Mailbox;
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Query<&Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Left;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.read() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.read() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.read() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation *= pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}

fn get_primary_window_size(windows: &Query<&Window>) -> Vec2 {
    let window = windows.get_single().expect("has main window");
    Vec2::new(window.width(), window.height())
}

/// Spawn a camera like this
fn load_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));
}

fn load_light(mut commands: Commands) {
    let light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 5000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(light);
}

fn load_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::BISQUE.into()),
        ..default()
    };

    commands.spawn(floor);
}

fn reload_data(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    bricks: Query<(Entity, &BrickDisplay)>,
    material_cache: ResMut<BrickColors>,
) {
    let mut target = None;

    if input.just_pressed(KeyCode::I) {
        target = Some(include_str!("../input.txt"));
    } else if input.just_pressed(KeyCode::E) {
        target = Some(include_str!("../example.txt"));
    }

    if let Some(data) = target {
        for e in bricks.iter() {
            commands.entity(e.0).despawn();
        }
        load_data(
            data,
            input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]),
            commands,
            meshes,
            materials,
            material_cache,
        );
    }
}

fn load_input(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    material_cache: ResMut<BrickColors>,
) {
    load_data(
        include_str!("../example.txt"),
        false,
        commands,
        meshes,
        materials,
        material_cache,
    );
}

const SCALE: f32 = 0.2;

fn load_data(
    data: &str,
    drop: bool,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_cache: ResMut<BrickColors>,
) {
    let mut bricks = aoc22::parse_input(data);
    if drop {
        let b = Building::new(bricks);
        bricks = b.bricks;
    }

    for brick in bricks {
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
        const DELTA: f32 = 1.0;
        let lower = Vec3::new(x.0 - DELTA, y.0 - DELTA, z.0 - DELTA) * SCALE;
        let upper = Vec3::new(x.1, y.1, z.1) * SCALE;

        let item = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(lower, upper))),
            material: material_cache.get(&mut materials, brick.idx),
            ..default()
        };
        commands.spawn((BrickDisplay {}, item));
    }
}

fn handle_exit(input: Res<Input<KeyCode>>, mut quit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::Q) {
        quit.send(AppExit);
    }
}
