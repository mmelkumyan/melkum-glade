use bevy::prelude::*;

use crate::spline::SplineDraft;
use crate::terrain::ground_hit;

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, place_point);
    }
}

fn place_point(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut spline: ResMut<SplineDraft>,
) {
    // Unpack cam/transform
    let (camera, camera_transform) = *camera_query;

    // Get click position
    if mouse.just_pressed(MouseButton::Left) {
        let Some(click_pos) = window.cursor_position() else {
            return;
        };

        let Some(hit_point) = ground_hit(camera, camera_transform, click_pos) else {
            return;
        };

        // Record pos
        spline.points.push(hit_point);
    }
}
