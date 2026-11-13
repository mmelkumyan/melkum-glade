use bevy::prelude::*;
use bevy_panorbit_camera::*;
use bevy::color::palettes::css::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod spline;
use spline::*;

mod terrain;
use terrain::*;


fn main() {
    // PLUGINS
    App::new().add_plugins((
        DefaultPlugins,
        PanOrbitCameraPlugin,
        EguiPlugin::default(),
        SplinePlugin,
        TerrainPlugin
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
        place_point
    ))

    // ADD RESOURCES
    .init_resource::<SplineDraft>()
    .run();
}

// Set up the scene with everything it needs on startup
fn setup(mut cmds: Commands) {
    // Spawn camera
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

    // Spawn light
    cmds.spawn((
        Name::new("Light"),
        DirectionalLight::default(),
        Transform::from_xyz(4., 10., 4.)
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // // Spawn cool cube
    // cmds.spawn((
    //     Name::new("Cube"),
    //     Mesh3d(meshes.add(Cuboid::default())),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: BLUE.into(),
    //         ..Default::default()
    //     })),
    //     Transform::from_xyz(0., 2., 0.)
    // ));
}



fn test_system(q: Query<&Window>) {
    let cnt = q.iter().count();
    println!("{cnt}");
}

// Click the screen and place a point
fn place_point(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
    // mut last_point: Local<Option<Vec3>>
    mut spline: ResMut<SplineDraft>
) {
    // Unpack cam/transform
    let (camera, camera_transform) = *camera_query;

    // Get click position
    if mouse.just_pressed(MouseButton::Left) {
        // Get click pos in screenspace
        let Some(click_pos) = window.cursor_position() else {return;};

        let Some(hit_point) = ground_hit(camera, camera_transform, click_pos) else {
            return;
        };

        // Record pos   
        spline.points.push(hit_point);
    }

    // Draw pts in spline
    for p in spline.points.iter() {
        gizmos.sphere(Isometry3d::new(
            p + Dir3::Y * 0.1, 
            Quat::default()), 
            0.2, 
            Color::WHITE
        );
    }
}
