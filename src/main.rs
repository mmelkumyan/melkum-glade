use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_panorbit_camera::*;
use bevy::color::palettes::css::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy::mesh::VertexAttributeValues;
use noise_functions::*;

// Spline resource 

#[derive(Resource, Default)]
struct SplineDraft {
    points: Vec<Vec3>,
}

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
        draw_grid,
        place_point,
        draw_spline
    ))

    // ADD RESOURCES
    .init_resource::<SplineDraft>()
    .run();
}

// Set up the scene with everything it needs on startup
fn setup(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
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

    // Generate ground mesh
    let mut ground = Plane3d::default()
        .mesh()
        .size(64.0, 64.0)
        .subdivisions(64)
        .build();
    let noise = Perlin.seed(10).frequency(0.05);
    let height = 6.0;
    if let Some(VertexAttributeValues::Float32x3(positions)) = ground.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in positions.iter_mut() {
            pos[1] = noise.sample2([pos[0], pos[2]]) * height;
        }
    }
    ground.compute_normals();

    // Spawn ground
    cmds.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(ground)),
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

// Draw a grid gizmo in the scene
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

        // Create ray from camera -> click pos
        let Ok(ray) = camera.viewport_to_world(camera_transform, click_pos) else {return;};

        // Get intersection point with ground
        let Some(hit_point) = ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {return;};

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

fn draw_spline(spline: Res<SplineDraft>, mut gizmos: Gizmos) {
    let Ok(curve) = CubicCardinalSpline::new_catmull_rom(spline.points.clone()).to_curve() else {return;};
    gizmos.linestrip(curve.iter_positions(100), Color::srgb(1., 0., 0.));
}