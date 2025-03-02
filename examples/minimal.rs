use bevy::{
    prelude::*,
    window::PresentMode::{self},
};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_transform_gizmo::TransformGizmoPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(TransformGizmoPlugin::new(
            Quat::from_rotation_y(-0.2), // Align the gizmo to a different coordinate system.
                                         // Use TransformGizmoPlugin::default() to align to the
                                         // scene's coordinate system.
        ))
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane, can't be selected
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
                material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
                ..Default::default()
            },
            bevy_transform_gizmo::GizmoTransformable,
        ))
        .insert(bevy_transform_gizmo::PickingBlocker);

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        bevy_transform_gizmo::GizmoTransformable,
    ));

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        bevy_transform_gizmo::GizmoPickSource::default(),
    ));
}
