use rand::{Rng};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use std::ops::Range;
use bevy::diagnostic::{PrintDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};

struct Mover;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(moving_system.system())
        .add_system(move_camera.system())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system())
        .run();
}

fn move_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
        let divider: f32 = 1.5;  // increase divider to slow down camera movement
        // Moving camera closer-further loop
        let translation = Vec3::new(
            -5.0 + 10.0 * (time.time_since_startup().as_secs_f32() / divider).sin(),
            5.0 * (time.time_since_startup().as_secs_f32() / divider).sin(),
            10.0 + 10.0 * (time.time_since_startup().as_secs_f32() / divider).cos(),
        );
        // Moving camera around
        // let translation = Vec3::new(
        //     10.0 * (time.time_since_startup().as_secs_f32() / divider).sin(),
        //     5.0 * (time.time_since_startup().as_secs_f32() / divider).sin(),
        //     // 0.0
        //     10.0 * (time.time_since_startup().as_secs_f32() / divider).cos(),
        // );
        transform.translation = translation;
        transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    }
}

fn moving_system(time: Res<Time>, mut query: Query<&mut Transform, With<Mover>>) {
    for mut transform in query.iter_mut() {
        // Another version of "wave"
        // let val = 1.0
        //     + 2.0 * ((time.time_since_startup().as_secs_f32() + transform.translation.z).sin()
        //     + (time.time_since_startup().as_secs_f32() + transform.translation.x).cos());

        // Major "wave"
        let val = 1.0
            + 5.0 * ((0.2 * (5.0 * time.time_since_startup().as_secs_f32() + transform.translation.z + transform.translation.x)).sin().abs());
        transform.scale = Vec3::new(1.0, val, 1.0);
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..Default::default()
    })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(6.0, 3.0, 4.0)),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(-6.0, 2.0, 0.0)),
            ..Default::default()
        })
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-5.0, 5.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 3.0, 0.0),
            )),
            ..Default::default()
        });

    let mut rng = rand::thread_rng();

    let bar_row_count = 40;
    let start = Vec3::new(-4.0, 0.0, -4.0);
    let mut x: f32 = start.x;
    let step: f32 = 0.20;
    let grey_color = Color::rgb(0.7, 0.7, 0.7);

    for _i in 1..bar_row_count {
        let mut z: f32 = start.z;
        for _k in 1..bar_row_count {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(
                        <bevy::render::mesh::shape::Box>::new(0.05, rng.gen_range(Range { start: 0.6, end: 0.9}), 0.05 ))),
                    material: materials.add(grey_color.into()),
                    transform: Transform::from_translation(Vec3::new(x, 0.0, z)),
                    ..Default::default()
                })
                .with(Mover)
            ;
            z += step;
        }
        x += step;
    }
}
