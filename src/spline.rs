use bevy::prelude::*;

// Spline resource 
#[derive(Resource, Default)]
pub struct SplineDraft {
    pub points: Vec<Vec3>,
}

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SplineDraft>()
            .add_systems(Update, draw_spline);
    }
}

fn build_curve(points: &[Vec3]) -> Option<CubicCurve<Vec3>> {
    return CubicCardinalSpline::new_catmull_rom(points.iter().copied()).to_curve().ok();
}

fn draw_spline(spline: Res<SplineDraft>, mut gizmos: Gizmos) {
    let SPLINE_SAMPLES = 100;
    let SPLINE_COLOR = Color::srgb(1., 0., 0.);

    let Some(curve) = build_curve(&spline.points) else {
        return;
    };
    gizmos.linestrip(curve.iter_positions(SPLINE_SAMPLES), SPLINE_COLOR);
}