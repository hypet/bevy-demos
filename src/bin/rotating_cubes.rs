use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .run();
}

struct Rotator;

fn rotator_system(_time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    let mut rotate_forward = 1.0;
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x((rotate_forward * 0.05f32.to_radians()) as f32);
        rotate_forward *= -1.0;
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_color = Color::rgb(0.7, 0.7, 0.7);
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..Default::default()
    })
        // cubes
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.7 })),
            material: materials.add(cube_color.into()),
            transform: Transform::from_translation(Vec3::new(-2.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(Rotator)

        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.7 })),
            material: materials.add(cube_color.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(Rotator)

        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.7 })),
            material: materials.add(cube_color.into()),
            transform: Transform::from_translation(Vec3::new(2.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(Rotator)

        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle  {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-3.0, 5.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
    ;
}