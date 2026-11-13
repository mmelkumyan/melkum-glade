use bevy::prelude::*;

use crate::spline::SplineDraft;
use crate::terrain::{Ground, ground_hit};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, place_point);
    }
}

fn place_point(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<Entity, With<Ground>>,
    mut ray_cast: MeshRayCast,
    mut spline: ResMut<SplineDraft>,
    
) {
    // Unpack cam/transform
    let (cam, cam_transform) = *camera_query;

    // Get click position
    if mouse.just_pressed(MouseButton::Left) {
        let Some(click_pos) = window.cursor_position() else {
            return;
        };
        let Ok(ray) = cam.viewport_to_world(cam_transform, click_pos) else {
            return;
        };
        let Some(hit_point) = ground_hit(ray, &mut ray_cast, *ground) else {
            return;
        };

        // Record pos
        spline.points.push(hit_point);
    }
}