use bevy::prelude::*;
use bevy_panorbit_camera::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins( PanOrbitCameraPlugin).
            add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut cmds: Commands) {
    cmds.spawn((
        Name::new("Main Camera"),
        Camera3d::default(), 
        PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Middle,
            ..default()
        },
        Transform::from_xyz(0., 5., 10.).looking_at(Vec3::ZERO, Vec3::Y)
    ));

    cmds.spawn((
        Name::new("Light"),
        DirectionalLight::default(),
        Transform::from_xyz(4., 10., 4.)
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));
}