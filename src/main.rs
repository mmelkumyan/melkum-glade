use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_panorbit_camera::*;
use bevy::color::palettes::css::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};


fn main() {
    // PLUGINS
    App::new().add_plugins((
        DefaultPlugins,
        PanOrbitCameraPlugin,
        EguiPlugin::default()
    ))
    .add_plugins(
        WorldInspectorPlugin::default(),
    )

    // STARTUP
    .add_systems(
    Startup, (
        test_system, 
        setup,
    ))

    // UPDATE
    .add_systems(
    Update, (
        draw_grid
    ))
    .run();
}

fn setup(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Spawn camera
    cmds.spawn((
        Name::new("Main Camera"),
        Camera3d::default(), 
        PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Right,
            modifier_pan: Some(KeyCode::ControlLeft),
            ..default()
        },
        Transform::from_xyz(0., 5., 10.).looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // Spawn light
    cmds.spawn((
        Name::new("Light"),
        DirectionalLight::default(),
        Transform::from_xyz(4., 10., 4.)
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // Spawn ground
    cmds.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default()
            .mesh().size(64.0, 64.0).subdivisions(64)
        )),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.2, 0.15, 0.1, 1.0),
            ..Default::default()
        })),
    ));

    // Spawn cool cube
    cmds.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLUE.into(),
            ..Default::default()
        })),
        Transform::from_xyz(0., 2., 0.)
    ));
}

fn draw_grid(mut gizmos: Gizmos) {
    gizmos.grid(
        Isometry3d::new(
            Vec3::new(0., 0.1, 0.),
            Quat::from_rotation_x(PI/2.)
        ),
        UVec2::new(32, 32),
        Vec2::splat(1.),
        Color::srgba(1., 0.5, 0., 0.5),
    );
}

fn test_system(q: Query<&Window>) {
    let cnt = q.iter().count();
    println!("{cnt}");
}